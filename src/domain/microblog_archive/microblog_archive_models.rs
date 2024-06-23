use chrono::{DateTime, Utc};

use crate::domain::models::{media::Media, tag::Tag};

#[derive(Debug, Clone)]
pub struct MicroblogArchivePost {
    slug: String,
    date: DateTime<Utc>,
    content: String,
    tags: Vec<Tag>,
    media: Vec<Media>,
}

impl MicroblogArchivePost {
    pub fn new(slug: String, date: DateTime<Utc>, content: String, tags: Vec<Tag>) -> Self {
        Self {
            slug,
            date,
            content,
            tags,
            media: vec![],
        }
    }
    
    pub fn with_media(mut self, media: Vec<Media>) -> Self {
        self.media = media;
        self
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn permalink(&self) -> String {
        format!("/micros/{}", self.slug)
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }
}