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
    pub media: Vec<Media>,
    pub tags: Vec<Tag>,
    pub updated_at: DateTime<Utc>,
}

impl MicroPost {
    pub fn new(
        slug: Slug,
        date: DateTime<Utc>,
        content: String,
        media: Vec<Media>,
        tags: Vec<Tag>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            slug,
            date,
            content,
            media,
            tags,
            updated_at,
        }
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
    }
}
