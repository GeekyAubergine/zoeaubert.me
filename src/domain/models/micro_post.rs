use std::fmt;

use super::image::Image;
use super::tag::Tag;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::media::Media;
use super::slug::Slug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroPost {
    pub slug: Slug,
    pub date: DateTime<Utc>,
    pub content: String,
    pub media: Vec<Media>,
    pub tags: Vec<Tag>,

    pub original_data_hash: u64,
}

impl MicroPost {
    pub fn new(
        slug: Slug,
        date: DateTime<Utc>,
        content: String,
        media: Vec<Media>,
        tags: Vec<Tag>,
        original_data_hash: u64,
    ) -> Self {
        Self {
            slug,
            date,
            content,
            media,
            tags,
            original_data_hash,
        }
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
    }
}
