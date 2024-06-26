use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use async_trait::async_trait;
use tracing::info;

use crate::{
    application::events::Event,
    domain::{
        mastodon_posts::mastodon_posts_models::{
            MastodonPost, MastodonPostNonSpoiler, MastodonPostSpoiler,
        },
        models::media::{image::Image, Media},
    },
    get_json,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
        config::Config,
    },
    prelude::Result,
    ONE_DAY_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_DAY_CACHE_PERIOD;

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
}

const MASTODON_PAGINATION_LIMIT: u32 = 40;

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

async fn mastodon_status_to_post(status: MastodonStatus) -> Result<MastodonPost> {
    let mut media = vec![];

    for post in status.media_attachments.iter() {
        match post {
            MastodonStatusMedia::Image {
                url,
                preview_url,
                description,
                meta,
                blurhash,
            } => {
                if let Some(description) = description {
                    let image =
                        Image::new(url, description, meta.original.width, meta.original.height);
                    media.push(Media::Image(image));
                }
            }
        }
    }

    let post = match status.spoiler_text {
        None => MastodonPost::NonSpoiler(MastodonPostNonSpoiler::new(
            status.id,
            status.uri,
            status.created_at,
            status.content,
            media,
            status.reblogs_count,
            status.favourites_count,
            status.replies_count,
        )),
        Some(spoiler_text) => MastodonPost::Spoiler(MastodonPostSpoiler::new(
            status.id,
            status.uri,
            status.created_at,
            status.content,
            media,
            status.reblogs_count,
            status.favourites_count,
            status.replies_count,
            spoiler_text,
        )),
    };

    Ok(post)
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
            return Ok(());
        }

        let statuses = fetch_pages(app_state.config()).await?;

        for status in statuses {
            if let Ok(post) = mastodon_status_to_post(status).await {
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
