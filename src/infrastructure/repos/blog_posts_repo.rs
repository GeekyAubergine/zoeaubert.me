use crate::{domain::models::{blog_post::BlogPost, tag::Tag}, error::Error, prelude::*};

use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::info;

use crate::domain::models::media::image::Image;

#[derive(Debug, Clone, Default)]
pub struct BlogPostsRepo {
    blog_posts: Arc<RwLock<HashMap<String, BlogPost>>>,
}

impl BlogPostsRepo {
    pub async fn commit(&self, blog_post: BlogPost) {
        let mut blog_posts_ref = self.blog_posts.write().await;
        blog_posts_ref.insert(blog_post.slug().to_owned(), blog_post);
    }

    pub async fn get_all(&self) -> HashMap<String, BlogPost> {
        self.blog_posts.read().await.clone()
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<BlogPost> {
        self.blog_posts.read().await.get(slug).cloned()
    }

    pub async fn get_all_by_published_date(&self) -> Vec<BlogPost> {
        let mut blog_posts = self.blog_posts.read().await.clone();

        let mut blog_posts = blog_posts
            .drain()
            .map(|(_, blog_post)| blog_post)
            .collect::<Vec<BlogPost>>();

        blog_posts.sort_by_key(|b| std::cmp::Reverse(*b.date()));

        blog_posts
    }

    pub async fn get_all_by_tag(&self, tag: &str) -> Vec<BlogPost> {
        let blog_posts = self.blog_posts.read().await.clone();

        blog_posts
            .values()
            .filter(|blog_post| {
                blog_post
                    .tags()
                    .iter()
                    .any(|blog_post_tag| blog_post_tag.tag() == tag)
            })
            .cloned()
            .collect()
    }
}
