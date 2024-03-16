use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub fn new(
        url: &str,
        alt: &str,
        width: u32,
        height: u32,
        title: Option<String>,
        description: Option<String>,
        date: Option<DateTime<Utc>>,
        parent_permalink: Option<String>,
    ) -> Self {
        Self {
            url: url.to_string(),
            alt: alt.to_string(),
            width,
            height,
            orientation: ImageOrientation::from_dimensions(width, height),
            title,
            description,
            date,
            parent_permalink,
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

    pub fn title(&self) -> Option<&str> {
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
}
