use crate::{domain::models::{image::Image, tag::Tag}, error::Error, prelude::*};

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
}

impl BlogPost {
    pub fn new(
        slug: String,
        date: DateTime<Utc>,
        title: String,
        description: String,
        tags: Vec<Tag>,
        hero_image: Option<Image>,
        content: String,
    ) -> Self {
        Self {
            slug: slug.to_string(),
            date,
            title: title.to_string(),
            description: description.to_string(),
            tags,
            hero_image,
            content: content.to_string(),
        }
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

    pub fn tags(&self) -> Vec<Tag> {
        self.tags.clone()
    }

    pub fn hero_image(&self) -> Option<Image> {
        self.hero_image.clone()
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
