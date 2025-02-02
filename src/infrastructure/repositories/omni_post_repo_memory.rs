use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, RwLockWriteGuard};
use tracing::warn;

use crate::{
    domain::{
        models::{
            content::Content,
            movie::{MovieId, MovieReview},
            omni_post::OmniPost,
            slug::Slug,
            tag::Tag,
        },
        repositories::{MovieReviewsRepo, OmniPostRepo},
        services::MovieService,
        state::State,
    },
    prelude::*,
};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct OmniPostRepoData {
    posts: HashMap<String, OmniPost>,
    post_date_order: Vec<String>,
    posts_by_tag: HashMap<Tag, Vec<String>>,
}

pub struct OmniPostRepoMemory {
    data: Arc<RwLock<OmniPostRepoData>>,
}

impl OmniPostRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(OmniPostRepoData::default())),
        }
    }

    pub fn update_internal_state(
        &self,
        mut data: RwLockWriteGuard<'_, OmniPostRepoData>,
    ) -> Result<()> {
        let mut posts = data.posts.values().cloned().collect::<Vec<OmniPost>>();

        posts.sort_by(|a, b| b.date().cmp(&a.date()));

        data.post_date_order = posts.iter().map(|p| p.key()).collect();

        for post in posts {
            for tag in post.tags() {
                data.posts_by_tag
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(post.key());
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl OmniPostRepo for OmniPostRepoMemory {
    async fn find_all_by_date(&self) -> Result<Vec<OmniPost>> {
        let data = self.data.read().await;

        let posts = data
            .post_date_order
            .iter()
            .filter_map(|slug| data.posts.get(slug))
            .cloned()
            .collect::<Vec<OmniPost>>();

        Ok(posts)
    }

    async fn find_all_by_tag(&self, tag: &Tag) -> Result<Vec<OmniPost>> {
        let data = self.data.read().await;

        let posts = data
            .posts_by_tag
            .get(tag)
            .map(|slugs| {
                slugs
                    .iter()
                    .filter_map(|slug| data.posts.get(slug))
                    .cloned()
                    .collect::<Vec<OmniPost>>()
            })
            .unwrap_or_default();

        Ok(posts)
    }

    async fn commit(&self, state: &impl State, posts: Vec<OmniPost>) -> Result<()> {
        let mut data = self.data.write().await;

        for post in posts {
            data.posts.insert(post.key(), post);
        }

        self.update_internal_state(data)?;

        Ok(())
    }
}
