use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    url: String,
    alt: String,
    width: u32,
    height: u32,
    orientation: ImageOrientation,
    title: Option<String>,
    description: Option<String>,
    date: Option<DateTime<Utc>>,
    parent_permalink: Option<String>,
}

impl Image {
    pub fn new(url: &str, alt: &str, width: u32, height: u32) -> Self {
        Self {
            url: url.to_string(),
            alt: alt.to_string(),
            width,
            height,
            orientation: ImageOrientation::from_dimensions(width, height),
            title: None,
            description: None,
            date: None,
            parent_permalink: None,
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

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn date(&self) -> Option<&DateTime<Utc>> {
        self.date.as_ref()
    }

    pub fn parent_permalink(&self) -> Option<&str> {
        self.parent_permalink.as_deref()
    }
}
