use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::image::Image;

use super::media::Media;

#[derive(Debug, Clone)]
pub struct MicroPost {
    pub slug: String,
    pub date: DateTime<Utc>,
    pub content: String,
    pub images: Vec<Image>,
    pub updated_at: DateTime<Utc>,
}

impl MicroPost {
    pub fn new(
        slug: String,
        date: DateTime<Utc>,
        content: String,
        images: Vec<Image>,
    ) -> Self {
        Self {
            slug,
            date,
            content,
            images,
            updated_at: Utc::now(),
        }
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn permalink(&self) -> String {
        format!("/micros/{}", self.slug)
    }

    pub fn media(&self) -> Vec<Media> {
        self.images
            .iter()
            .map(|image| image.into())
            .collect::<Vec<Media>>()
    }
}
