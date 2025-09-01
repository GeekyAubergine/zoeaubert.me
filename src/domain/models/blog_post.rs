use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{image::Image, media::Media, page::Page, slug::Slug, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub slug: Slug,
    pub date: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub tags: Vec<Tag>,
    pub hero_image: Option<Image>,
    pub content: String,
    pub media: Vec<Media>,
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

    pub fn with_hero_image(mut self, hero_image: Image) -> Self {
        self.hero_image = Some(hero_image);
        self
    }

    pub fn with_media(mut self, media: Vec<Media>) -> Self {
        self.media = media;
        self
    }

    pub fn with_images(mut self, images: Vec<Image>) -> Self {
        self.media = images.into_iter().map(Media::from_image).collect();
        self
    }

    pub fn permalink(&self) -> String {
        self.slug.permalink_string()
    }

    pub fn page(&self) -> Page {
        let mut page = Page::new(
            self.slug.clone(),
            Some(&self.title),
            Some(self.description.clone()),
        )
        .with_date(self.date)
        .with_tags(self.tags.clone());

        if let Some(image) = &self.hero_image {
            page = page.with_image(image.clone().into());
        }

        page
    }
}
