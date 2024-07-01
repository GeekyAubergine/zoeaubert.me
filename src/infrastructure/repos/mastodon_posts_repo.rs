use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::mastodon_post::MastodonPost;

#[derive(Debug, Clone, Default)]
pub struct MastodonPostsRepo {
    posts: Arc<RwLock<HashMap<String, MastodonPost>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl MastodonPostsRepo {
    pub async fn rebuild_from_archive(&self, archive: MastodonPostsRepoArchive) {
        let mut posts = self.posts.write().await;
        let mut last_updated = self.last_updated.write().await;

        *posts = archive.posts;
        *last_updated = archive.last_updated;
    }

    pub async fn commit(&self, post: MastodonPost) {
        let mut posts = self.posts.write().await;
        posts.insert(post.id().to_owned(), post);

        let mut last_updated = self.last_updated.write().await;
        *last_updated = Utc::now();
    }

    pub async fn get_last_updated(&self) -> DateTime<Utc> {
        *self.last_updated.read().await
    }

    pub async fn get_archived(&self) -> MastodonPostsRepoArchive {
        let posts = self.posts.read().await;

        MastodonPostsRepoArchive {
            posts: posts.clone(),
            last_updated: *self.last_updated.read().await,
        }
    }

    pub async fn get_post(&self, id: &str) -> Option<MastodonPost> {
        let posts = self.posts.read().await;

        posts.get(id).cloned()
    }

    pub async fn get_all_posts(&self) -> HashMap<String, MastodonPost> {
        let posts = self.posts.read().await;

        posts
            .iter()
            .map(|(key, post)| (key.to_owned(), post.clone()))
            .collect()
    }

    pub async fn get_posts_by_most_recently_created(&self) -> Vec<MastodonPost> {
        let posts = self.posts.read().await;

        let mut posts_array = posts.values().cloned().collect::<Vec<MastodonPost>>();

        posts_array.sort_by(|a, b| b.created_at().cmp(a.created_at()));

        posts_array
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostsRepoArchive {
    pub posts: HashMap<String, MastodonPost>,
    pub last_updated: DateTime<Utc>,
}
