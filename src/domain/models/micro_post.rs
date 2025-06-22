use std::fmt;

use super::image::Image;
use super::page::Page;
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
    pub description: Option<String>,
    pub media: Vec<Media>,
    pub tags: Vec<Tag>,
}

impl MicroPost {
    pub fn new(
        slug: Slug,
        date: DateTime<Utc>,
        content: String,
        description: Option<String>,
        media: Vec<Media>,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            slug,
            date,
            content,
            description,
            media,
            tags,
        }
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink()
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }

    pub fn optimised_media(&self) -> &Vec<Media> {
        &self.media
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn page(&self) -> Page {
        let mut page = Page::new(self.slug.clone(), None, self.description.clone())
            .with_date(self.date)
            .with_tags(self.tags.clone());

        if let Some(first) = self.media.first() {
            match first {
                Media::Image(image) => {
                    page = page.with_image(image.clone().into());
                }
            }
        }

        page
    }
}
