use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{
        models::{
            movie::{MovieId, MovieReview},
            slug::Slug,
        },
        repositories::MovieReviewsRepo,
    },
    prelude::*,
};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct MovieReviewsRepoData {
    movie_reviews: HashMap<Slug, MovieReview>,
}

pub struct MovieReviewsRepoMemory {
    data: Arc<RwLock<MovieReviewsRepoData>>,
}

impl MovieReviewsRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(MovieReviewsRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl MovieReviewsRepo for MovieReviewsRepoMemory {
    async fn find_by_movie_id(&self, movie_id: &MovieId) -> Result<Vec<MovieReview>> {
        let data = self.data.read().await;
        let movie_reviews = data
            .movie_reviews
            .values()
            .filter(|review| review.movie.id == *movie_id)
            .cloned()
            .collect();
        Ok(movie_reviews)
    }

    async fn find_all_grouped_by_movie_id(&self) -> Result<HashMap<MovieId, Vec<MovieReview>>> {
        let data = self.data.read().await;
        let mut movie_reviews = HashMap::new();
        for review in data.movie_reviews.values() {
            let entry = movie_reviews
                .entry(review.movie.id)
                .or_insert_with(Vec::new);
            entry.push(review.clone());
        }

        Ok(movie_reviews)
    }

    async fn commit(&self, movie_review: &MovieReview) -> Result<()> {
        let mut data = self.data.write().await;
        data.movie_reviews
            .insert(movie_review.post.slug(), movie_review.clone());
        Ok(())
    }
}
