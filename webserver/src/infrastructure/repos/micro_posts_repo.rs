use std::{collections::HashMap, hash::Hash, sync::Arc};

use tokio::sync::RwLock;

use crate::domain::models::micro_post::MicroPost;

#[derive(Debug, Clone, Default)]
pub struct MicroPostsRepo {
    micro_posts: Arc<RwLock<HashMap<String, MicroPost>>>,
}

impl MicroPostsRepo {
    pub async fn commit(&self, micro_post: MicroPost) {
        let mut micro_posts_ref = self.micro_posts.write().await;
        micro_posts_ref.insert(micro_post.slug().to_owned(), micro_post);
    }

    pub async fn get_all(&self) -> HashMap<String, MicroPost> {
        self.micro_posts.read().await.clone()
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<MicroPost> {
        self.micro_posts.read().await.get(slug).cloned()
    }

    pub async fn get_all_by_published_date(&self) -> Vec<MicroPost> {
        let mut micro_posts = self.micro_posts.read().await.clone();

        let mut micro_posts = micro_posts
            .drain()
            .map(|(_, micro_post)| micro_post)
            .collect::<Vec<MicroPost>>();

        micro_posts.sort_by_key(|b| std::cmp::Reverse(*b.date()));

        micro_posts
    }

    pub async fn get_all_by_tag(&self, tag: &str) -> Vec<MicroPost> {
        let micro_posts = self.micro_posts.read().await.clone();

        micro_posts
            .values()
            .filter(|micro_post| {
                micro_post
                    .tags()
                    .iter()
                    .any(|micro_post_tag| micro_post_tag.tag() == tag)
            })
            .cloned()
            .collect()
    }
}