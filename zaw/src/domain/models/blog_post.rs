use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{image::{Image, ImageUuid}, media::{Media, MediaUuid}, slug::Slug, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: Slug,
    pub date: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub tags: Vec<Tag>,
    pub hero_image: Option<ImageUuid>,
    pub content: String,
    pub media: Vec<MediaUuid>,
}

impl BlogPost {
    pub fn new(
        slug: Slug,
        date: DateTime<Utc>,
        title: String,
        description: String,
        tags: Vec<Tag>,
        content: String,
    ) -> Self {
        Self {
            slug,
            date,
            title,
            description,
            tags,
            hero_image: None,
            content,
            media: vec![],
        }
    }

    pub fn with_hero_image(mut self, hero_image: ImageUuid) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    pub fn with_media(mut self, media: Vec<MediaUuid>) -> Self {
        self.media = media;
        self
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
    }
}
