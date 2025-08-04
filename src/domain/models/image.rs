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
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub original: SizedImage,
    pub large: SizedImage,
    pub small: SizedImage,
    pub description: String,
    pub link_on_click: Option<Slug>,
}

impl Image {
    pub fn orientation(&self) -> MediaOrientation {
        self.original.dimensions.orientation()
    }

    // pub fn original(&self) -> RenderableImage {
    //     RenderableImage {
    //         url: self.original.file.as_cdn_url(),
    //         dimensions: &self.original.dimensions,
    //         description: &self.description,
    //     }
    // }

    // pub fn large(&self) -> RenderableImage {
    //     RenderableImage {
    //         url: self.large.file.as_cdn_url(),
    //         dimensions: &self.large.dimensions,
    //         description: &self.description,
    //     }
    // }

    // pub fn small(&self) -> RenderableImage {
    //     RenderableImage {
    //         url: self.small.file.as_cdn_url(),
    //         dimensions: &self.small.dimensions,
    //         description: &self.description,
    //     }
    // }
}

// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
// pub struct LegacyImage {
//     pub path: File,
//     pub alt: String,
//     pub dimensions: MediaDimensions,
//     pub title: Option<String>,
//     pub description: Option<String>,
//     pub date: Option<DateTime<Utc>>,
//     pub parent_slug: Option<Slug>,
//     pub updated_at: DateTime<Utc>,
// }

// impl LegacyImage {
//     pub fn new(path: &File, alt: &str, dimensions: &MediaDimensions) -> Self {
//         Self {
//             path: path.clone(),
//             alt: alt.to_string(),
//             dimensions: *dimensions,
//             title: None,
//             description: None,
//             date: None,
//             parent_slug: None,
//             updated_at: Utc::now(),
//         }
//     }

//     pub fn with_title(&self, title: &str) -> Self {
//         Self {
//             title: Some(title.to_string()),
//             ..self.clone()
//         }
//     }

//     pub fn with_description(&self, description: &str) -> Self {
//         Self {
//             description: Some(description.to_string()),
//             ..self.clone()
//         }
//     }

//     pub fn with_date(&self, date: &DateTime<Utc>) -> Self {
//         Self {
//             date: Some(date.clone()),
//             ..self.clone()
//         }
//     }

//     pub fn with_parent_slug(&self, parent_slug: &Slug) -> Self {
//         Self {
//             parent_slug: Some(parent_slug.clone()),
//             ..self.clone()
//         }
//     }

//     pub fn with_updated_at(&self, updated_at: DateTime<Utc>) -> Self {
//         Self {
//             updated_at,
//             ..self.clone()
//         }
//     }

//     pub fn title(&self) -> &str {
//         match &self.title {
//             Some(title) => title,
//             None => &self.alt,
//         }
//     }

//     pub fn title_inner(&self) -> Option<&str> {
//         self.title.as_deref()
//     }

//     pub fn description(&self) -> Option<&str> {
//         self.description.as_deref()
//     }

//     pub fn date(&self) -> Option<&DateTime<Utc>> {
//         self.date.as_ref()
//     }

//     pub fn parent_slug(&self) -> Option<Slug> {
//         self.parent_slug.clone()
//     }

//     pub fn cdn_url(&self) -> Url {
//         if self.path.starts_with("http") {
//             return self.path.as_url().unwrap().unwrap();
//         }

//         let path = format!("{}/{}", dotenv!("CDN_URL"), self.path.to_string_lossy());

//         path.parse().unwrap()
//     }

//     pub fn orientation(&self) -> MediaOrientation {
//         self.dimensions.orientation()
//     }
// }
