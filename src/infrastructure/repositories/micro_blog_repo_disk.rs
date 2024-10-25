use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::MicroPostsRepo;
use crate::domain::services::FileService;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

const FILE_NAME: &str = "micro_posts.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MicroPostsRepoData {
    micro_posts: HashMap<Slug, MicroPost>,
}

impl MicroPostsRepoData {
    pub fn new() -> Self {
        Self {
            micro_posts: HashMap::new(),
        }
    }
}

pub struct MicroPostsRepoDisk {
    data: Arc<RwLock<MicroPostsRepoData>>,
    file_service: FileServiceDisk,
}

impl MicroPostsRepoDisk {
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
impl MicroPostsRepo for MicroPostsRepoDisk {
    async fn find_all(&self) -> Result<Vec<MicroPost>> {
        let data = self.data.read().await;

        let mut posts = data
            .micro_posts
            .values()
            .cloned()
            .collect::<Vec<MicroPost>>();

        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<MicroPost>> {
        let data = self.data.read().await;

        Ok(data.micro_posts.get(slug).cloned())
    }

    async fn commit(&self, micro_post: &MicroPost) -> Result<()> {
        let mut data = self.data.write().await;

        data.micro_posts
            .insert(micro_post.slug.clone(), micro_post.clone());

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
