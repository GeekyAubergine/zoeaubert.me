use serde::{Deserialize, Serialize};

use super::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Media {
    Image(Image),
}

impl Media {
    pub fn from_image(image: Image) -> Self {
        Media::Image(image)
    }
}

impl From<Image> for Media {
    fn from(image: Image) -> Self {
        Media::from_image(image)
    }
}

impl From<&Image> for Media {
    fn from(image: &Image) -> Self {
        Media::from_image(image.clone())
    }
}
