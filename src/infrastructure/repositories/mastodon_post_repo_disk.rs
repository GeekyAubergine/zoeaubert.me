use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{
        models::mastodon_post::MastodonPost, repositories::MastodonPostsRepo, services::FileService,
    },
    infrastructure::services::file_service_disk::FileServiceDisk,
};

use crate::prelude::*;

const FILE_NAME: &str = "mastodon_posts.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MastodonPostRepoData {
    mastodon_posts: HashMap<String, MastodonPost>,
    updated_at: Option<DateTime<Utc>>,
}

pub struct MastodonPostRepoDisk {
    data: Arc<RwLock<MastodonPostRepoData>>,
    file_service: FileServiceDisk,
}

impl MastodonPostRepoDisk {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl MastodonPostsRepo for MastodonPostRepoDisk {
    async fn find_all_by_date(&self) -> Result<Vec<MastodonPost>> {
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

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
