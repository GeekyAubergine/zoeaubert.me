use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::image::{Image, ImageUuid};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MediaUuid(Uuid);

impl MediaUuid {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for MediaUuid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl fmt::Display for MediaUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ImageUuid> for MediaUuid {
    fn from(uuid: ImageUuid) -> Self {
        Self::new(uuid.into())
    }
}

impl From<&ImageUuid> for MediaUuid {
    fn from(uuid: &ImageUuid) -> Self {
        Self::new(uuid.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Media {
    Image(Image),
}

impl Media {
    pub fn from_image(image: Image) -> Self {
        Media::Image(image)
    }

    pub fn uuid(&self) -> MediaUuid {
        match self {
            Media::Image(image) => image.uuid().into(),
        }
    }
}
