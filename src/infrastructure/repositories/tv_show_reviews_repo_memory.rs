use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{
        models::{
            movie::{MovieId, MovieReview},
            slug::Slug, tv_show::{TvShowId, TvShowReview},
        },
        repositories::{MovieReviewsRepo, TvShowReviewsRepo},
    },
    prelude::*,
};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TvShowReviewsRepoData {
    tv_show_reviews: HashMap<Slug, TvShowReview>,
}

pub struct TvShowReviewsRepoMemory {
    data: Arc<RwLock<TvShowReviewsRepoData>>,
}

impl TvShowReviewsRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(TvShowReviewsRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl TvShowReviewsRepo for TvShowReviewsRepoMemory {
    async fn find_by_tv_show_id(&self, movie_id: &TvShowId) -> Result<Vec<TvShowReview>> {
        let data = self.data.read().await;
        let movie_reviews = data
            .tv_show_reviews
            .values()
            .filter(|review| review.tv_show.id == *movie_id)
            .cloned()
            .collect();
        Ok(movie_reviews)
    }

    async fn find_all_grouped_by_tv_show_id(&self) -> Result<HashMap<TvShowId, Vec<TvShowReview>>> {
        let data = self.data.read().await;
        let mut movie_reviews = HashMap::new();
        for review in data.tv_show_reviews.values() {
            let entry = movie_reviews
                .entry(review.tv_show.id)
                .or_insert_with(Vec::new);
            entry.push(review.clone());
        }

        Ok(movie_reviews)
    }

    async fn commit(&self, tv_show_review: &TvShowReview) -> Result<()> {
        let mut data = self.data.write().await;
        data.tv_show_reviews
            .insert(tv_show_review.post.slug(), tv_show_review.clone());
        Ok(())
    }
}
