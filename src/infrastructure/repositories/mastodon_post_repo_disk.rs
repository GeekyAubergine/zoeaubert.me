use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{models::mastodon_post::MastodonPost, repositories::MastodonPostsRepo},
    infrastructure::utils::file_system::{
        make_archive_file_path, read_json_file_or_default, write_json_file,
    },
};

use crate::prelude::*;

const FILE_NAME: &str = "mastodon_posts.json";

fn make_file_path() -> PathBuf {
    make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MastodonPostRepoData {
    mastodon_posts: HashMap<String, MastodonPost>,
    updated_at: Option<DateTime<Utc>>,
}

pub struct MastodonPostRepoDisk {
    data: Arc<RwLock<MastodonPostRepoData>>,
}

impl MastodonPostRepoDisk {
    pub async fn new() -> Result<Self> {
        let data = read_json_file_or_default(&make_file_path()).await?;

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
            .mastodon_posts
            .values()
            .cloned()
            .collect::<Vec<MastodonPost>>();

        posts.sort_by(|a, b| b.created_at().cmp(&a.created_at()));

        Ok(posts)
    }

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>> {
        let data = self.data.read().await;

        Ok(data.updated_at)
    }

    async fn commit(&self, post: &MastodonPost) -> Result<()> {
        let mut data = self.data.write().await;

        data.mastodon_posts
            .insert(post.id().to_string(), post.clone());
        data.updated_at = Some(Utc::now());

        write_json_file(&make_file_path(), &data.clone()).await?;

        Ok(())
    }
}
