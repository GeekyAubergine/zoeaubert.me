use chrono::{DateTime, Duration, Utc};
use dotenvy_macro::dotenv;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use tracing::debug;
use url::Url;

use crate::domain::models::image::Image;
use crate::domain::models::mastodon_post::{
    MastodonPost, MastodonPostNonSpoiler, MastodonPostSpoiler,
};
use crate::domain::models::media::Media;
use crate::domain::models::tag::Tag;
use crate::domain::queries::mastodon_queries::{
    commit_mastodon_post, find_all_mastodon_posts, find_mastodon_posts_last_updated_at,
};
use crate::domain::repositories::Profiler;
use crate::domain::services::{CacheService, CdnService};
use crate::domain::state::State;
use crate::infrastructure::utils::file_system::make_file_path_from_date_and_file;
use crate::infrastructure::utils::image_utils::image_from_url;
use crate::infrastructure::utils::networking::*;
use crate::{prelude::*, ONE_HOUR_PERIOD};

const SELF_URL: &str = "zoeaubert.me";
const APPLICATIONS_TO_IGNORE: [&str; 2] = ["Micro.blog", "status.lol"];

static TAGS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<a[^>]*rel="tag">#<span>(.*?)</span></a>"#).unwrap());

static EMPTY_P_TAGS: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<p>\s*</p>"#).unwrap());

#[derive(Debug, Deserialize)]
struct MastodonStatusMediaImageSizes {
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
struct MastodonStatusMediaImageMeta {
    original: MastodonStatusMediaImageSizes,
    small: MastodonStatusMediaImageSizes,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum MastodonStatusMedia {
    #[serde(rename = "image")]
    Image {
        url: Url,
        preview_url: Option<Url>,
        description: Option<String>,
        meta: MastodonStatusMediaImageMeta,
        blurhash: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
struct MastodonStatusApplication {
    name: String,
    website: Option<Url>,
}

#[derive(Debug, Deserialize)]
struct MastodonStatusTag {
    name: String,
    url: Url,
}

#[derive(Debug, Deserialize)]
struct MastodonStatus {
    id: String,
    uri: Url,
    created_at: DateTime<Utc>,
    content: String,
    media_attachments: Vec<MastodonStatusMedia>,
    reblogs_count: u32,
    favourites_count: u32,
    replies_count: u32,
    application: Option<MastodonStatusApplication>,
    visibility: Option<String>,
    spoiler_text: Option<String>,
    tags: Vec<MastodonStatusTag>,
}

const MASTODON_PAGINATION_LIMIT: u32 = 40;

static API_BASE_URL: Lazy<String> = Lazy::new(|| {
    format!(
        "https://social.lol/api/v1/accounts/{}/statuses?exclude_reblogs=true&exclude_replies=true&limit={}",
        dotenv!("MASTODON_ACCOUNT_ID"),
        MASTODON_PAGINATION_LIMIT,
    )
});

struct PostIdBoundaries {
    min_id: Option<String>,
    max_id: Option<String>,
}

impl PostIdBoundaries {
    fn new() -> Self {
        Self {
            min_id: None,
            max_id: None,
        }
    }

    fn with_min_id(mut self, min_id: String) -> Self {
        self.set_min_id(min_id);

        self
    }

    fn with_max_id(mut self, max_id: String) -> Self {
        self.set_max_id(max_id);

        self
    }

    fn set_min_id(&mut self, min_id: String) {
        self.min_id = Some(min_id);
    }

    fn set_max_id(&mut self, max_id: String) {
        self.max_id = Some(max_id);
    }

    fn add_to_url(&self, url: &Url) -> Url {
        let mut url = url.clone();

        if let Some(min_id) = &self.min_id {
            url.query_pairs_mut().append_pair("min_id", min_id);
        }

        if let Some(max_id) = &self.max_id {
            url.query_pairs_mut().append_pair("max_id", max_id);
        }

        url
    }
}

async fn fetch_page(
    client: &reqwest::Client,
    boundaries: &PostIdBoundaries,
) -> Result<Vec<MastodonStatus>> {
    let url = boundaries.add_to_url(&API_BASE_URL.parse().unwrap());

    download_json(client, &url).await
}

async fn fetch_pages(
    client: &reqwest::Client,
    mut boundaries: PostIdBoundaries,
) -> Result<Vec<MastodonStatus>> {
    let mut statuses = Vec::new();

    loop {
        let page = fetch_page(client, &boundaries).await?;

        if page.is_empty() {
            break;
        }

        statuses.extend(page);

        if statuses.len() < MASTODON_PAGINATION_LIMIT as usize {
            break;
        }

        boundaries.set_max_id(statuses.last().unwrap().id.clone());
    }

    Ok(statuses)
}

fn extract_tags(content: &str) -> Vec<String> {
    TAGS_REGEX
        .captures_iter(content)
        .map(|capture| capture.get(1).unwrap().as_str().to_string())
        .collect()
}

fn strip_tags(content: &str) -> String {
    let content = TAGS_REGEX.replace_all(content, "").to_string();

    EMPTY_P_TAGS.replace_all(&content, "").to_string()
}

async fn mastodon_status_to_post(
    state: &impl State,
    status: MastodonStatus,
) -> Result<Option<MastodonPost>> {
    state.profiler().post_processed().await?;

    if let Some(application) = &status.application {
        if APPLICATIONS_TO_IGNORE.contains(&application.name.as_str()) {
            return Ok(None);
        }
    }

    if status.content.contains(SELF_URL) {
        return Ok(None);
    }

    let tags = extract_tags(&status.content)
        .iter()
        .map(|t| Tag::from_string(t))
        .collect();

    let mut content = strip_tags(&status.content);

    let mut post = match status.spoiler_text {
        None => MastodonPost::NonSpoiler(MastodonPostNonSpoiler::new(
            status.id,
            status.uri,
            status.created_at,
            content,
            tags,
        )),
        Some(spoiler_text) => MastodonPost::Spoiler(MastodonPostSpoiler::new(
            status.id,
            status.uri,
            status.created_at,
            content,
            spoiler_text,
            tags,
        )),
    };

    for attachment in status.media_attachments.iter() {
        match attachment {
            MastodonStatusMedia::Image {
                url,
                preview_url,
                description,
                meta,
                blurhash,
            } => {
                if let Some(description) = description {
                    let url_path = Path::new(url.path());

                    let file_name = url_path.file_name().unwrap().to_str().unwrap();

                    let path =
                        make_file_path_from_date_and_file(&status.created_at, &file_name, None);

                    let image = image_from_url(state, &url, &path, &description)
                        .await?
                        .with_date(post.created_at())
                        .with_parent_slug(&post.slug());

                    let preview = match preview_url {
                        Some(preview_url) => {
                            let url_path = Path::new(preview_url.path());

                            let file_name = url_path.file_name().unwrap().to_str().unwrap();

                            let path = make_file_path_from_date_and_file(
                                &status.created_at,
                                file_name,
                                Some("preview"),
                            );

                            let preview_image =
                                image_from_url(state, &preview_url, &path, &description)
                                    .await?
                                    .with_date(post.created_at())
                                    .with_parent_slug(&post.slug());

                            Some(preview_image)
                        }
                        None => None,
                    };

                    let image = Media::from_image(image);
                    let preview = preview.map(|p| Media::from_image(p));

                    post.add_media(image, preview);
                }
            }
        }
    }

    Ok(Some(post))
}

pub async fn update_mastodon_posts_command(state: &impl State) -> Result<()> {
    let last_updated = find_mastodon_posts_last_updated_at(state).await?;

    if let Some(last_updated) = find_mastodon_posts_last_updated_at(state).await? {
        if last_updated + ONE_HOUR_PERIOD > Utc::now() {
            return Ok(());
        }
    }

    let posts = find_all_mastodon_posts(state).await?;

    let mut oldest_post_to_update_since: Option<&MastodonPost> = None;

    // Only update posts that have been created or edited within the last two weeks
    let two_weeks_ago = Utc::now() - Duration::weeks(2);

    for post in posts.iter() {
        if post.created_at() < &two_weeks_ago {
            break;
        }

        oldest_post_to_update_since = Some(post);
    }

    // If there is not post within the last two weeks, use the newest post
    if oldest_post_to_update_since.is_none() {
        oldest_post_to_update_since = posts.first();
    }

    let min_id = oldest_post_to_update_since.map(|p| p.id().to_string());

    let client = reqwest::Client::new();

    let mut boundaries = PostIdBoundaries::new();

    if let Some(min_id) = min_id {
        boundaries.set_min_id(min_id);
    }

    let statuses = fetch_pages(&client, boundaries).await?;

    for status in statuses.into_iter() {
        let post = mastodon_status_to_post(state, status).await?;

        if let Some(post) = post {
            commit_mastodon_post(state, &post).await?;
        }
    }

    Ok(())
}
