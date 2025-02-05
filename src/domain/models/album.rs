use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::infrastructure::utils::cover_photos_for_album::cover_photos_for_album;

use super::{image::Image, page::Page, slug::Slug, tag::Tag};

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
            Some(&self.small_image.alt),
        )
        .with_image(self.small_image.clone().into())
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
    pub original_data_hash: u64,
}

impl Album {
    pub fn new(
        slug: Slug,
        title: String,
        description: Option<String>,
        date: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        original_data_hash: u64,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            date,
            photos: vec![],
            updated_at,
            original_data_hash,
        }
    }

    pub fn add_photo(&mut self, photo: AlbumPhoto) {
        self.photos.push(photo);
    }

    pub fn cover_images(&self) -> Vec<Image> {
        let cover_photos = cover_photos_for_album(&self);

        let cover_images = cover_photos
            .into_iter()
            .map(|photo| photo.small_image.clone())
            .collect::<Vec<_>>();

        cover_images
    }

    pub fn page(&self) -> Page {
        let description = self.description.clone().unwrap_or("".to_string());

        let mut page = Page::new(self.slug.clone(), Some(&self.title), Some(&description))
            .with_date(self.date);

        let cover_images = self.cover_images();

        if let Some(cover_image) = cover_images.first() {
            page = page.with_image(cover_image.clone().into());
        }

        page
    }
}
