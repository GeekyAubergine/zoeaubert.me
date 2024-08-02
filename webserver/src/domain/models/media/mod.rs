use image::Image;
use serde::{Deserialize, Serialize};

pub mod image;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Media {
    Image(Image),
}

impl Media {
    pub fn from_image(image: Image) -> Self {
        Media::Image(image)
    }
}