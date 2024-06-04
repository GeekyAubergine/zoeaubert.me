use std::{
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{get_json, infrastructure::config::Config, prelude::*, ONE_HOUR_CACHE_PERIOD};

use super::status_lol_models::StatusLolPost;

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

#[derive(Debug, Clone, Default)]
pub struct StatusLolRepo {
    posts: Arc<RwLock<Vec<ReponseStatusLolPost>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl StatusLolRepo {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(Vec::new())),
            last_updated: Arc::new(RwLock::new(UNIX_EPOCH.into())),
        }
    }

    pub async fn load_from_archive(&self, archive: StatusLolRepoArchive) {
        let mut posts = self.posts.write().await;
        *posts = archive.posts;

        let mut last_updated = self.last_updated.write().await;
        *last_updated = archive.last_updated;
    }

    pub async fn reload(&self, config: &Config) -> Result<()> {
        let response = get_json::<StatusLolResponse>(config.status_lol().url()).await?;

        let posts = response.response.statuses;

        let mut post_ref = self.posts.write().await;
        *post_ref = posts.clone();

        let mut last_updated_ref = self.last_updated.write().await;

        *last_updated_ref = Utc::now();

        Ok(())
    }

    pub async fn get_archived(&self) -> StatusLolRepoArchive {
        let posts = self.posts.read().await;
        let last_updated = *self.last_updated.read().await;

        StatusLolRepoArchive {
            posts: posts.clone(),
            last_updated,
        }
    }

    pub async fn get_all(&self) -> Vec<StatusLolPost> {
        let posts = self.posts.read().await;
        posts
            .iter()
            .map(|post| post.clone().into())
            .collect::<Vec<StatusLolPost>>()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLolRepoArchive {
    posts: Vec<ReponseStatusLolPost>,
    last_updated: DateTime<Utc>,
}
