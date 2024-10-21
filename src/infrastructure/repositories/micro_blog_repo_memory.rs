use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::{BlogPostsRepo, MicroPostsRepo};
use crate::prelude::*;

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

pub struct MicroPostsRepoMemory {
    data: Arc<RwLock<MicroPostsRepoData>>,
}

impl MicroPostsRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(MicroPostsRepoData::new())),
        }
    }
}

#[async_trait::async_trait]
impl MicroPostsRepo for MicroPostsRepoMemory {
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

        Ok(())
    }
}
