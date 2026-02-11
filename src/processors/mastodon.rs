use std::{collections::HashMap, path::Path, thread::sleep, time::Duration};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::{
    config::CONFIG, domain::models::{
        mastodon_post::{MastodonPost, MastodonPostNonSpoiler, MastodonPostSpoiler, MastodonPosts},
        media::Media,
        tag::Tag,
    }, prelude::*, processors::tasks::{Task, run_tasks}, services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile, WritableFile},
        media_service::MediaService,
    }
};

const FILE_NAME: &str = "mastodon_posts.json";
const QUERY: &str = "mastodon";
const SELF_URL: &str = "zoeaubert.me/blog";
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
    edited_at: Option<DateTime<Utc>>,
}

const MASTODON_PAGINATION_LIMIT: u32 = 40;

static API_BASE_URL: Lazy<String> = Lazy::new(|| {
    format!(
        "https://social.lol/api/v1/accounts/{}/statuses?exclude_reblogs=true&exclude_replies=true&limit={}",
        CONFIG.mastodon.account_id,
        MASTODON_PAGINATION_LIMIT,
    )
});

fn fetch_page(ctx: &ServiceContext, oldest: Option<&String>) -> Result<Vec<MastodonStatus>> {
    let mut url: Url = API_BASE_URL.parse().unwrap();

    if let Some(since) = oldest {
        url.query_pairs_mut().append_pair("max_id", since);
    }

    ctx.network.download_json(&url)
}

fn fetch_most_recent(ctx: &ServiceContext) -> Result<Vec<MastodonStatus>> {
    fetch_page(ctx, None)
}

fn fetch_all(ctx: &ServiceContext) -> Result<Vec<MastodonStatus>> {
    let mut statuses: Vec<MastodonStatus> = Vec::new();

    loop {
        let oldest = statuses.last().map(|s| &s.id);

        let page = fetch_page(ctx, oldest)?;

        if page.is_empty() {
            break;
        }

        statuses.extend(page);

        if statuses.len() < MASTODON_PAGINATION_LIMIT as usize {
            break;
        }

        sleep(Duration::from_secs(5));
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

struct ProcessStatus {
    status: MastodonStatus,
}

impl Task for ProcessStatus {
    type Output = Option<MastodonPost>;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        if let Some(application) = &self.status.application {
            if APPLICATIONS_TO_IGNORE.contains(&application.name.as_str()) {
                return Ok(None);
            }
        }

        if self.status.content.contains(SELF_URL) {
            return Ok(None);
        }

        info!("Processing Mastodon post {:?}", &self.status.created_at);

        let tags = extract_tags(&self.status.content)
            .iter()
            .map(|t| Tag::from_string(t))
            .collect();

        let mut content = strip_tags(&self.status.content);

        let mut post = match self.status.spoiler_text {
            None => MastodonPost::NonSpoiler(MastodonPostNonSpoiler::new(
                self.status.id,
                self.status.uri,
                self.status.created_at,
                content,
                tags,
                self.status.edited_at.unwrap_or(self.status.created_at),
            )),
            Some(spoiler_text) => MastodonPost::Spoiler(MastodonPostSpoiler::new(
                self.status.id,
                self.status.uri,
                self.status.created_at,
                content,
                spoiler_text,
                tags,
                self.status.edited_at.unwrap_or(self.status.created_at),
            )),
        };

        for attachment in self.status.media_attachments.iter() {
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

                        let cdn_file = CdnFile::from_date_and_file_name(
                            &self.status.created_at,
                            &file_name,
                            None,
                        );

                        let image = MediaService::image_from_url(
                            ctx,
                            &url,
                            &cdn_file,
                            &description,
                            Some(&&post.slug().relative_string()),
                            Some(self.status.created_at),
                        )?;

                        post.add_media(image.into());
                    }
                }
            }
        }

        Ok(Some(post))
    }
}

pub fn load_mastodon_posts(ctx: &ServiceContext) -> Result<MastodonPosts> {
    let file = FileService::archive(FILE_NAME.into());

    let mut posts: MastodonPosts = file.read_json_or_default()?;

    if !ctx
        .query_limiter
        .can_query_within_fifteen_minutes(QUERY)?
    {
        return Ok(posts);
    }

    info!("Fetching mastodon posts data");

    let statuses = match posts.count() {
        0 => fetch_all(ctx)?,
        _ => fetch_most_recent(ctx)?,
    };

    let tasks = statuses
        .into_iter()
        .map(|status| ProcessStatus { status })
        .collect();

    let results = run_tasks(tasks, ctx)?;

    for result in results {
        if let Some(post) = result {
            posts.add(post);
        }
    }

    file.write_json(&posts)?;

    Ok(posts)
}
