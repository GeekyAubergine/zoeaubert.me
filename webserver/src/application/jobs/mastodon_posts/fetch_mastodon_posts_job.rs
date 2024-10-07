use std::time::Duration;

use askama::filters::format;
use chrono::{DateTime, Datelike, Utc};
use regex::Regex;
use serde::Deserialize;

use async_trait::async_trait;
use tracing::info;
use uuid::Uuid;

use crate::{
    application::{
        events::Event, jobs::cdn::copy_file_from_internet_to_cdn_job::CopyFileFromInternetToCdnJob,
    },
    domain::models::{
        mastodon_post::{MastodonPost, MastodonPostNonSpoiler, MastodonPostSpoiler},
        media::{image::Image, Media},
        tag::Tag,
    },
    get_json,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::{Job, JobPriority},
        config::Config,
        services::cdn::CdnPath,
    },
    prelude::Result,
    ONE_DAY_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_DAY_CACHE_PERIOD;
const SELF_URL: &str = "zoeaubert.me";
const APPLICATIONS_TO_IGNORE: [&str; 2] = ["Micro.blog", "status.lol"];

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
        url: String,
        preview_url: Option<String>,
        description: Option<String>,
        meta: MastodonStatusMediaImageMeta,
        blurhash: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
struct MastodonStatusApplication {
    name: String,
    website: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MastodonStatusTag {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct MastodonStatus {
    id: String,
    uri: String,
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

lazy_static! {
    static ref TAGS_REGEX: Regex =
        // Regex::new(r#"<p><a[^>]*rel=\\"tag\\">#<span>(.*)</span></a></p>"#).unwrap();
        Regex::new(r#"<a[^>]*rel="tag">#<span>(.*?)</span></a>"#).unwrap();

        static ref EMPTY_P_TAGS: Regex = Regex::new(r#"<p>\s*</p>"#).unwrap();
}

fn make_statuses_url(config: &Config) -> String {
    format!(
        "https://social.lol/api/v1/accounts/{}/statuses?exclude_reblogs=true&exclude_replies=true&limit={}",
        config.mastodon().account_id(),
        MASTODON_PAGINATION_LIMIT
    )
}

async fn fetch_page(max_id: Option<String>, config: &Config) -> Result<Vec<MastodonStatus>> {
    let url = match max_id {
        Some(max_id) => format!("{}&max_id={}", make_statuses_url(config), max_id),
        None => make_statuses_url(config),
    };

    let statuses = get_json(&url).await?;

    Ok(statuses)
}

async fn fetch_pages(config: &Config) -> Result<Vec<MastodonStatus>> {
    let mut max_id = None;
    let mut all_statuses = vec![];

    loop {
        let statuses = fetch_page(max_id, config).await?;

        if statuses.is_empty() {
            break;
        }

        max_id = Some(statuses.last().unwrap().id.clone());
        all_statuses.extend(statuses);
    }

    Ok(all_statuses)
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
    app_state: &AppState,
    status: MastodonStatus,
) -> Result<Option<MastodonPost>> {
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
            status.reblogs_count,
            status.favourites_count,
            status.replies_count,
            tags,
        )),
        Some(spoiler_text) => MastodonPost::Spoiler(MastodonPostSpoiler::new(
            status.id,
            status.uri,
            status.created_at,
            content,
            status.reblogs_count,
            status.favourites_count,
            status.replies_count,
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
                    let file_name = url.split('/').last().unwrap();

                    let cdn_path = CdnPath::new(format!(
                        "/{}/{}/{}/{}",
                        status.created_at.year(),
                        status.created_at.month(),
                        status.created_at.day(),
                        file_name
                    ));

                    let image = Image::new(
                        &Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes()),
                        &cdn_path.url(app_state.config()),
                        description,
                        meta.original.width,
                        meta.original.height,
                    )
                    .with_date(*post.created_at())
                    .with_parent_permalink(&post.permalink());

                    let media = Media::Image(image);

                    post.add_media(media);

                    app_state
                        .dispatch_job(
                            CopyFileFromInternetToCdnJob::new(url.clone(), cdn_path),
                            JobPriority::Low,
                        )
                        .await;
                }
            }
        }
    }

    Ok(Some(post))
}

#[derive(Debug)]
pub struct FetchMastodonPostsJob;

impl FetchMastodonPostsJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for FetchMastodonPostsJob {
    fn name(&self) -> &str {
        "FetchMastodonPostsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let last_updated = app_state.mastodon_posts_repo().get_last_updated().await;

        if last_updated + NO_REFETCH_DURATION > Utc::now() {
            info!("Skipping fetching mastodon posts - cache is still valid");
            return Ok(());
        }
        info!("Fetching mastodon posts");

        let statuses = fetch_pages(app_state.config()).await?;

        for status in statuses {
            if let Ok(Some(post)) = mastodon_status_to_post(app_state, status).await {
                info!("Updating mastodon post: {}", post.id());
                app_state.mastodon_posts_repo().commit(post).await;
            }
        }

        app_state
            .dispatch_event(Event::MastodonPostsRepoUpdated)
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        application::jobs::mastodon_posts::fetch_mastodon_posts_job::{
            extract_tags, strip_tags, MastodonStatusTag,
        },
        domain::models::tag::Tag,
    };

    #[test]
    fn it_should_extract_tags() {
        let content = "<p>Everyone Is Awesome ❤\u{fe0f}</p><p>Set: 40516</p><p><a href=\"https://social.lol/tags/Lego\" class=\"mention hashtag\" rel=\"tag\">#<span>Lego</span></a></p>";

        let tags = extract_tags(content);

        let expected = vec![Tag::from_string("Lego")];

        let first = tags.first().unwrap();

        assert_eq!(first, &expected[0].tag());

        let content = r#"<p>I haven&#39;t even started the original one yet, and Lego are releasing another Disney castle that I&#39;m going to have to buy 🤣</p><p><a href="https://social.lol/tags/Lego" class="mention hashtag" rel="tag">#<span>Lego</span></a> <a href="https://social.lol/tags/Disney" class="mention hashtag" rel="tag">#<span>Disney</span></a></p>"#;

        let tags = extract_tags(content);

        let expected = vec![Tag::from_string("Lego"), Tag::from_string("Disney")];

        assert_eq!(tags[0], expected[0].tag());
        assert_eq!(tags[1], expected[1].tag());
    }

    #[test]
    fn it_should_strip_tags() {
        let content = "<p>Everyone Is Awesome ❤\u{fe0f}</p><p>Set: 40516</p><p><a href=\"https://social.lol/tags/Lego\" class=\"mention hashtag\" rel=\"tag\">#<span>Lego</span></a></p>";

        let stripped_content = strip_tags(content);

        let expected = "<p>Everyone Is Awesome ❤\u{fe0f}</p><p>Set: 40516</p>";

        assert_eq!(stripped_content, expected);

        let content = r#"<p>I haven&#39;t even started the original one yet, and Lego are releasing another Disney castle that I&#39;m going to have to buy 🤣</p><p><a href="https://social.lol/tags/Lego" class="mention hashtag" rel="tag">#<span>Lego</span></a> <a href="https://social.lol/tags/Disney" class="mention hashtag" rel="tag">#<span>Disney</span></a></p>"#;

        let stripped_content = strip_tags(content);

        let expected = "<p>I haven&#39;t even started the original one yet, and Lego are releasing another Disney castle that I&#39;m going to have to buy 🤣</p>";

        assert_eq!(stripped_content, expected);
    }
}
