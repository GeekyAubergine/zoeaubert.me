use std::{
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{get_json, infrastructure::config::Config, prelude::*, ONE_HOUR_CACHE_PERIOD};

use super::status_lol_models::StatusLolPost;


#[derive(Debug, Clone, Default)]
pub struct StatusLolRepo {
    posts: Arc<RwLock<Vec<StatusLolPost>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl StatusLolRepo {
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(Vec::new())),
            last_updated: Arc::new(RwLock::new(UNIX_EPOCH.into())),
        }
    }

    pub async fn rebuild_from_archive(&self, archive: StatusLolRepoArchive) {
        let mut posts = self.posts.write().await;
        *posts = archive.posts;

        let mut last_updated = self.last_updated.write().await;
        *last_updated = archive.last_updated;
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

    pub async fn commit(&self, posts: Vec<StatusLolPost>) {
        let mut post_ref = self.posts.write().await;
        *post_ref = posts.clone();

        let mut last_updated_ref = self.last_updated.write().await;
        *last_updated_ref = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLolRepoArchive {
    posts: Vec<StatusLolPost>,
    last_updated: DateTime<Utc>,
}
