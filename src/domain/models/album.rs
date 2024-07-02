use std::collections::HashMap;

use chrono::{DateTime, Datelike, Utc};

use super::{media::image::Image, tag::Tag};

#[derive(Debug, Clone)]
pub struct AlbumPhoto {
    image: Image,
    file_name: String,
    tags: Vec<Tag>,
    featured: bool,
}

impl AlbumPhoto {
    pub fn new(
        image: Image,
        file_name: String,
        tags: Vec<Tag>,
        featured: bool,
    ) -> Self {
        Self {
            image,
            file_name,
            tags,
            featured,
        }
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn featured(&self) -> bool {
        self.featured
    }

    pub fn permalink(&self, parent_album: &Album) -> String {
        format!("{}/{}", parent_album.permalink(), self.file_name)
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    title: String,
    description: Option<String>,
    date: DateTime<Utc>,
    photos: HashMap<String, AlbumPhoto>,
    photo_order: Vec<String>,
}

impl Album {
    pub fn new(title: String, description: Option<String>, date: DateTime<Utc>) -> Self {
        Self {
            title,
            description,
            date,
            photos: HashMap::new(),
            photo_order: Vec::new(),
        }
    }

    pub fn add_photo(&mut self, photo: AlbumPhoto) {
        let file_name = photo.file_name().to_string();
        self.photos.insert(file_name.clone(), photo);
        self.photo_order.push(file_name);
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn id(&self) -> String {
        format!(
            "{}-{}",
            self.date.format("%Y-%m-%d"),
            self.title
        )
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn photos_map(&self) -> &HashMap<String, AlbumPhoto> {
        &self.photos
    }

    pub fn photos_order(&self) -> &[String] {
        &self.photo_order
    }

    pub fn photos_ordered(&self) -> Vec<&AlbumPhoto> {
        self.photo_order
            .iter()
            .filter_map(|file_name| self.photos.get(file_name))
            .collect()
    }

    pub fn photo(&self, file_name: &str) -> Option<&AlbumPhoto> {
        self.photos.get(file_name)
    }

    pub fn photo_count(&self) -> usize {
        self.photos.len()
    }

    pub fn tags(&self) -> Vec<Tag> {
        self.photos
            .values()
            .flat_map(|photo| photo.tags().to_vec())
            .collect()
    }

    pub fn permalink(&self) -> String {
        format!(
            "/albums/{}/{}/{}",
            self.date.year(),
            self.date.month(),
            self.title
        )
    }
}
