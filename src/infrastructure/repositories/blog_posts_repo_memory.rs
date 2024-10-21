use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::BlogPostsRepo;
use crate::prelude::*;

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

pub struct BlogPostsRepoMemory {
    data: Arc<RwLock<BlogPostsRepoData>>,
}

impl BlogPostsRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(BlogPostsRepoData::new())),
        }
    }
}

#[async_trait::async_trait]
impl BlogPostsRepo for BlogPostsRepoMemory {
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

        Ok(())
    }
}
