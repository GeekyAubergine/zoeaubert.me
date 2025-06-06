use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models::{image::Image, page::Page, slug::Slug, tag::Tag},
    infrastructure::utils::cover_photos_for_album::cover_photos_for_album,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumPhoto {
    pub slug: Slug,
    pub description: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub small_image: Image,
    pub large_image: Image,
    pub original_image: Image,
    pub featured: bool,
    pub updated_at: DateTime<Utc>,
}

impl AlbumPhoto {
    pub fn new(
        slug: Slug,
        description: String,
        date: DateTime<Utc>,
        tags: Vec<Tag>,
        small_image: Image,
        large_image: Image,
        original_image: Image,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            slug,
            description,
            date,
            tags,
            small_image,
            large_image,
            original_image,
            featured: false,
            updated_at,
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
            Some(self.small_image.alt.clone()),
        )
        .with_image(self.small_image.clone().into())
    }
}
