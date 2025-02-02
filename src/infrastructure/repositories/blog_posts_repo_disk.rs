use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::BlogPostsRepo;
use crate::domain::services::FileService;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

const FILE_NAME: &str = "blog_posts.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct BlogPostsRepoData {
    blog_posts: HashMap<Slug, BlogPost>,
}

impl BlogPostsRepoData {
    pub fn new() -> Self {
        Self {
            blog_posts: HashMap::new(),
        }
    }
}

pub struct BlogPostsRepoDisk {
    data: Arc<RwLock<BlogPostsRepoData>>,
    file_service: FileServiceDisk,
}

impl BlogPostsRepoDisk {
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
impl BlogPostsRepo for BlogPostsRepoDisk {
    async fn find_all_by_date(&self) -> Result<Vec<BlogPost>> {
        let data = self.data.read().await;

        let mut posts = data.blog_posts.values().cloned().collect::<Vec<BlogPost>>();

        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<BlogPost>> {
        let data = self.data.read().await;

        Ok(data.blog_posts.get(slug).cloned())
    }

    async fn commit(&self, blog_post: &BlogPost) -> Result<()> {
        let mut data = self.data.write().await;

        data.blog_posts
            .insert(blog_post.slug.clone(), blog_post.clone());

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
