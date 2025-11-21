use std::{
    fmt,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::services::cdn_service::CdnFile;

use super::{
    media::{Media, MediaDimensions, MediaOrientation},
    slug::Slug,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SizedImage {
    pub file: CdnFile,
    pub dimensions: MediaDimensions,
}

pub struct RenderableImage<'l> {
    pub url: Url,
    pub dimensions: &'l MediaDimensions,
    pub description: &'l str,
    pub date: Option<&'l DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ImageLinkOnClick {
    InternalSlug
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub original: SizedImage,
    pub large: SizedImage,
    pub small: SizedImage,
    pub tiny: SizedImage,
    pub description: String,
    pub link_on_click: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

impl Image {
    pub fn orientation(&self) -> MediaOrientation {
        self.original.dimensions.orientation()
    }
}
