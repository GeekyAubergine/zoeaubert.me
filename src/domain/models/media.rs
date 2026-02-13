use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::models::image::Image;

// Had a few images where they're just a few pixels off square, so we want to catch them too or they look weird
const SQUARE_MEDIA_MARGIN_OF_ERROR: f32 = 0.1;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct MediaDimensions {
    pub width: u32,
    pub height: u32,
}

impl MediaDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn orientation(&self) -> MediaOrientation {
        MediaOrientation::from_dimensions(self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum MediaOrientation {
    Landscape,
    Portrait,
    Square,
}

impl MediaOrientation {
    pub fn from_dimensions(dimensions: &MediaDimensions) -> Self {
        let aspect_ratio = dimensions.aspect_ratio();

        if aspect_ratio > 1.0 + SQUARE_MEDIA_MARGIN_OF_ERROR {
            return Self::Landscape;
        }

        if aspect_ratio < 1.0 - SQUARE_MEDIA_MARGIN_OF_ERROR {
            return Self::Portrait;
        }

        Self::Square
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Landscape => "landscape",
            Self::Portrait => "portrait",
            Self::Square => "square",
        }
    }

    pub fn is_landscape(&self) -> bool {
        matches!(self, Self::Landscape)
    }

    pub fn is_portrait(&self) -> bool {
        matches!(self, Self::Portrait)
    }

    pub fn is_square(&self) -> bool {
        matches!(self, Self::Square)
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

    pub fn orientation(&self) -> MediaOrientation {
        match self {
            Media::Image(image) => image.orientation(),
        }
    }

    pub fn original_cdn_url(&self) -> Url {
        match self {
            Media::Image(image) => image.original.file.as_cdn_url(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_corretly_determine_the_orientation_of_media() {
        assert_eq!(
            MediaOrientation::from_dimensions(&MediaDimensions::new(3024, 4032)),
            MediaOrientation::Portrait
        );
        assert_eq!(
            MediaOrientation::from_dimensions(&MediaDimensions::new(4032, 3024)),
            MediaOrientation::Landscape
        );
        assert_eq!(
            MediaOrientation::from_dimensions(&MediaDimensions::new(3024, 3024)),
            MediaOrientation::Square
        );
    }
}
