use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models::{image::Image, page::Page, slug::Slug, tag::Tag},
    utils::cover_photos_for_album::cover_photos_for_album,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumPhoto {
    pub slug: Slug,
    pub description: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub image: Image,
    pub featured: bool,
}

impl AlbumPhoto {
    pub fn new(
        slug: Slug,
        description: String,
        date: DateTime<Utc>,
        tags: Vec<Tag>,
        image: Image,
    ) -> Self {
        Self {
            slug,
            description,
            date,
            tags,
            image,
            featured: false,
        }
    }

    pub fn set_featured(mut self, featured: bool) -> Self {
        self.featured = featured;
        self
    }

    pub fn page(&self) -> Page {
        Page::new(
            self.slug.clone(),
            Some(&self.description),
            Some(self.image.description.clone()),
        )
        .with_image(self.image.clone().into())
    }
}
