use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::image::Image;
use super::tag::Tag;

use super::media::Media;
use super::slug::Slug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroPost {
    pub slug: Slug,
    pub date: DateTime<Utc>,
    pub content: String,
    pub images: Vec<Image>,
    pub tags: Vec<Tag>,
}

impl MicroPost {
    pub fn new(
        slug: Slug,
        date: DateTime<Utc>,
        content: String,
        images: Vec<Image>,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            slug,
            date,
            content,
            images,
            tags,
        }
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
    }

    pub fn media(&self) -> Vec<Media> {
        self.images
            .iter()
            .map(|image| image.into())
            .collect::<Vec<Media>>()
    }
}
