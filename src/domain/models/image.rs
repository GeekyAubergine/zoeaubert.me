use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::services::cdn_service::CdnFile;

use super::media::{MediaDimensions, MediaOrientation};

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
    InternalSlug,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub original: SizedImage,
    pub large: SizedImage,
    pub small: SizedImage,
    pub description: String,
    pub link_on_click: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

impl Image {
    pub fn orientation(&self) -> MediaOrientation {
        self.original.dimensions.orientation()
    }
}
