use std::{
    fmt,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::{media::{Media, MediaDimensions, MediaOrientation}, slug::Slug};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub path: PathBuf,
    pub alt: String,
    pub dimensions: MediaDimensions,
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub parent_slug: Option<Slug>,
    pub updated_at: DateTime<Utc>,
}

impl Image {
    pub fn new(path: &Path, alt: &str, dimensions: &MediaDimensions) -> Self {
        Self {
            path: path.to_path_buf(),
            alt: alt.to_string(),
            dimensions: *dimensions,
            title: None,
            description: None,
            date: None,
            parent_slug: None,
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

    pub fn with_date(&self, date: &DateTime<Utc>) -> Self {
        Self {
            date: Some(date.clone()),
            ..self.clone()
        }
    }

    pub fn with_parent_slug(&self, parent_slug: &Slug) -> Self {
        Self {
            parent_slug: Some(parent_slug.clone()),
            ..self.clone()
        }
    }

    pub fn with_updated_at(&self, updated_at: DateTime<Utc>) -> Self {
        Self {
            updated_at,
            ..self.clone()
        }
    }

    pub fn title(&self) -> &str {
        match &self.title {
            Some(title) => title,
            None => &self.alt,
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

    pub fn parent_slug(&self) -> Option<Slug> {
        self.parent_slug.clone()
    }

    pub fn cdn_url(&self) -> Url {
        // Some legacy thing from microblog is causing headaches with paths getting double cdn
        let path_as_str = self.path.to_str().unwrap();

        if path_as_str.starts_with("http") {
            return path_as_str.parse().unwrap();
        }

        let path = self.path.strip_prefix("/").unwrap_or(&self.path);

        let path = format!("{}/{}", dotenv!("CDN_URL"), path.to_string_lossy());

        path.parse().unwrap()
    }

    pub fn orientation(&self) -> MediaOrientation {
        self.dimensions.orientation()
    }
}
