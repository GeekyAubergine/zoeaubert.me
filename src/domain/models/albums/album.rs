use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models::{albums::album_photo::AlbumPhoto, image::Image, page::Page, slug::Slug},
    utils::cover_photos_for_album::cover_photos_for_album,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub slug: Slug,
    pub title: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub photos: Vec<AlbumPhoto>,
}

impl Album {
    pub fn new(
        slug: Slug,
        title: String,
        description: Option<String>,
        date: DateTime<Utc>,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            date,
            photos: vec![],
        }
    }

    pub fn add_photo(&mut self, photo: AlbumPhoto) {
        self.photos.push(photo);
    }

    pub fn cover_images(&self) -> Vec<&Image> {
        let cover_photos = cover_photos_for_album(&self);

        let cover_images = cover_photos
            .into_iter()
            .map(|photo| &photo.image)
            .collect::<Vec<_>>();

        cover_images
    }

    pub fn page(&self) -> Page {
        let description = self.description.clone().unwrap_or("".to_string());

        let mut page =
            Page::new(self.slug.clone(), Some(&self.title), Some(description)).with_date(self.date);

        let cover_images = self.cover_images();

        if let Some(cover_image) = cover_images.first() {
            page = page.with_image(cover_image.clone().into());
        }

        page
    }

    pub fn index_of_photo(&self, photo: &AlbumPhoto) -> Option<usize> {
        self.photos.iter().position(|p| p.slug == photo.slug)
    }

    pub fn total_photos(&self) -> usize {
        self.photos.len()
    }

    pub fn previous_photo(&self, photo: &AlbumPhoto) -> Option<&AlbumPhoto> {
        if let Some(index) = self.index_of_photo(photo) {
            if index > 0 {
                return Some(&self.photos[index - 1]);
            }
        }

        None
    }

    pub fn next_photo(&self, photo: &AlbumPhoto) -> Option<&AlbumPhoto> {
        if let Some(index) = self.index_of_photo(photo) {
            if index < self.photos.len() - 1 {
                return Some(&self.photos[index + 1]);
            }
        }

        None
    }
}
