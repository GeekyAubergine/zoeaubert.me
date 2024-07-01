use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    application::events::Event, domain::models::status_lol::StatusLolPost, get_json, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result, ONE_HOUR_CACHE_PERIOD, STATUS_LOL_ARCHIVE_FILENAME
};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReponseStatusLolPost {
    id: String,
    address: String,
    created: String,
    relative_time: String,
    emoji: String,
    content: String,
    external_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatusLolResponseValue {
    message: String,
    statuses: Vec<ReponseStatusLolPost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatusLolRequestValue {
    status_code: u16,
    success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLolResponse {
    request: ResponseStatusLolRequestValue,
    response: ResponseStatusLolResponseValue,
}

impl From<ReponseStatusLolPost> for StatusLolPost {
    fn from(post: ReponseStatusLolPost) -> Self {
        let key = format!("statuslol-{}", post.id);
        let permalink = format!("/micros/statuslol-{}", post.id);
        let original_url = format!("https://{}.status.lol/{}", post.address, post.id);

        let date = match post.created.parse::<i64>() {
            Ok(date) => match DateTime::from_timestamp(date, 0) {
                Some(date) => date,
                None => Utc::now(),
            },
            Err(_) => Utc::now(),
        };

        StatusLolPost::new(key, permalink, date, post.content, post.emoji, original_url)
    }
}

#[derive(Debug)]
pub struct FetchStatusLolPostsJob;
impl FetchStatusLolPostsJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for FetchStatusLolPostsJob {
    fn name(&self) -> &str {
        "FetchStatusLolPostsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Fetching status.lol posts");
        let response = get_json::<StatusLolResponse>(app_state.config().status_lol().url()).await?;

        let posts = response.response.statuses;

        let posts = posts
            .iter()
            .map(|post| post.clone().into())
            .collect::<Vec<StatusLolPost>>();

        app_state.status_lol_repo().commit(posts).await;

        app_state
            .dispatch_event(Event::StatusLolRepoUpdated)
            .await?;

        Ok(())
    }
}
