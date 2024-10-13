use crate::{
    domain::models::{media::image::Image, media::Media, tag::Tag},
    error::Error,
    prelude::*,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    slug: String,
    date: DateTime<Utc>,
    title: String,
    description: String,
    tags: Vec<Tag>,
    hero_image: Option<Image>,
    content: String,
    media: Vec<Media>,
}

impl BlogPost {
    pub fn new(
        slug: String,
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

    pub fn with_hero_image(mut self, hero_image: Image) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    pub fn with_media(mut self, media: Vec<Media>) -> Self {
        self.media = media;
        self
    }

    pub fn permalink(&self) -> String {
        format!("/blog/{}", self.slug)
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn hero_image(&self) -> Option<Image> {
        self.hero_image.clone()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }
}
