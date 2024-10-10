use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{image::ImageUuid, media::MediaUuid};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MicroPostUuid(pub Uuid);

impl MicroPostUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for MicroPostUuid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<MicroPostUuid> for Uuid {
    fn from(micro_post_uuid: MicroPostUuid) -> Self {
        micro_post_uuid.0
    }
}

impl From<&MicroPostUuid> for Uuid {
    fn from(micro_post_uuid: &MicroPostUuid) -> Self {
        micro_post_uuid.0
    }
}

impl fmt::Display for MicroPostUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct MicroPost {
    pub uuid: MicroPostUuid,
    pub slug: String,
    pub date: DateTime<Utc>,
    pub content: String,
    pub image_order: Vec<ImageUuid>,
    pub updated_at: DateTime<Utc>,
}

impl MicroPost {
    pub fn new(
        uuid: MicroPostUuid,
        slug: String,
        date: DateTime<Utc>,
        content: String,
        image_order: Vec<ImageUuid>,
    ) -> Self {
        Self {
            uuid,
            slug,
            date,
            content,
            image_order,
            updated_at: Utc::now(),
        }
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn permalink(&self) -> String {
        format!("/micros/{}", self.slug)
    }

    pub fn media_order(&self) -> Vec<MediaUuid> {
        self
            .image_order
            .iter()
            .map(|image_uuid| image_uuid.into())
            .collect::<Vec<MediaUuid>>()
    }
}
