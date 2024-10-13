use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{models::mastodon_post::MastodonPost, repositories::MastodonPostsRepo},
    infrastructure::utils::file_system::{make_archive_file_path, read_json_file_or_default, write_json_file},
};

use crate::prelude::*;

const FILE_NAME: &str = "mastodon_posts.json";

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MastodonPostRepoData {
    blog_posts: HashMap<String, MastodonPost>,
}

impl MastodonPostRepoData {
    pub fn new() -> Self {
        Self {
            blog_posts: HashMap::new(),
        }
    }
}

pub struct MastodonPostRepoDisk {
    data: Arc<RwLock<MastodonPostRepoData>>,
}

impl MastodonPostRepoDisk {
    pub async fn new() -> Result<Self> {
        let data = read_json_file_or_default(&make_archive_file_path(FILE_NAME)).await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
        })
    }
}

#[async_trait::async_trait]
impl MastodonPostsRepo for MastodonPostRepoDisk {
    async fn find_all(&self) -> Result<Vec<MastodonPost>> {
        let data = self.data.read().await;

        let mut posts = data
            .blog_posts
            .values()
            .cloned()
            .collect::<Vec<MastodonPost>>();

        posts.sort_by(|a, b| b.created_at().cmp(&a.created_at()));

        Ok(posts)
    }

    async fn commit(&self, post: &MastodonPost) -> Result<()> {
        let mut data = self.data.write().await;

        data.blog_posts.insert(post.id().to_string(), post.clone());

        write_json_file(&make_archive_file_path(FILE_NAME), &data.clone()).await?;

        Ok(())
    }
}
