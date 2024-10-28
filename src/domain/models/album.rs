use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::infrastructure::utils::cover_photos_for_album::cover_photos_for_album;

use super::{image::Image, slug::Slug, tag::Tag};

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
        }
    }

    pub fn set_featured(mut self, featured: bool) -> Self {
        self.featured = featured;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub slug: Slug,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub photos: Vec<AlbumPhoto>,
    pub updated_at: DateTime<Utc>,
}

impl Album {
    pub fn new(
        slug: Slug,
        title: String,
        description: Option<String>,
        date: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            date,
            photos: vec![],
            updated_at,
        }
    }

    pub fn add_photo(&mut self, photo: AlbumPhoto) {
        self.photos.push(photo);
    }
}
