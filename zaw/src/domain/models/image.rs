use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::media::Media;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ImageOrientation {
    Landscape,
    Portrait,
    Square,
}

impl ImageOrientation {
    pub fn from_dimensions(width: u32, height: u32) -> Self {
        match width.cmp(&height) {
            std::cmp::Ordering::Greater => Self::Landscape,
            std::cmp::Ordering::Less => Self::Portrait,
            std::cmp::Ordering::Equal => Self::Square,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Landscape => "landscape",
            Self::Portrait => "portrait",
            Self::Square => "square",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImageUuid(pub Uuid);

impl ImageUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for ImageUuid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<ImageUuid> for Uuid {
    fn from(image_uuid: ImageUuid) -> Self {
        image_uuid.0
    }
}

impl From<&ImageUuid> for Uuid {
    fn from(image_uuid: &ImageUuid) -> Self {
        image_uuid.0
    }
}

impl fmt::Display for ImageUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub uuid: ImageUuid,
    pub url: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    pub orientation: ImageOrientation,
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub parent_permalink: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl Image {
    pub fn new(uuid: &ImageUuid, url: &str, alt: &str, width: u32, height: u32) -> Self {
        Self {
            uuid: *uuid,
            url: url.to_string(),
            alt: alt.to_string(),
            width,
            height,
            orientation: ImageOrientation::from_dimensions(width, height),
            title: None,
            description: None,
            date: None,
            parent_permalink: None,
            updated_at: Utc::now(),
        }
    }

    pub fn with_title(&self, title: &str) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self.clone()
        }
    }

    pub fn with_description(&self, description: &str) -> Self {
        Self {
            description: Some(description.to_string()),
            ..self.clone()
        }
    }

    pub fn with_date(&self, date: DateTime<Utc>) -> Self {
        Self {
            date: Some(date),
            ..self.clone()
        }
    }

    pub fn with_parent_permalink(&self, parent_permalink: &str) -> Self {
        Self {
            parent_permalink: Some(parent_permalink.to_string()),
            ..self.clone()
        }
    }

    pub fn with_updated_at(&self, updated_at: DateTime<Utc>) -> Self {
        Self {
            updated_at,
            ..self.clone()
        }
    }

    pub fn uuid(&self) -> &ImageUuid {
        &self.uuid
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn orientation(&self) -> &ImageOrientation {
        &self.orientation
    }

    pub fn title(&self) -> &str {
        match &self.title {
            Some(title) => title,
            None => self.alt(),
        }
    }

    pub fn title_inner(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn date(&self) -> Option<&DateTime<Utc>> {
        self.date.as_ref()
    }

    pub fn parent_permalink(&self) -> Option<&str> {
        self.parent_permalink.as_deref()
    }

    pub fn is_landscape(&self) -> bool {
        self.orientation == ImageOrientation::Landscape
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.height as f32 / self.width as f32
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl From<Image> for Media {
    fn from(image: Image) -> Self {
        Media::Image(image)
    }
}

impl From<&Image> for Media {
    fn from(image: &Image) -> Self {
        Media::Image(image.clone())
    }
}
