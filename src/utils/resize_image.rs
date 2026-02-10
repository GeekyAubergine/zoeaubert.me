use image::{DynamicImage, GenericImageView};

use crate::domain::models::media::{MediaDimensions, MediaOrientation};

const LANDSCAPE_LARGE_IMAGE_WIDTH: u32 = 2000;
const LANDSCAPE_SMALL_IMAGE_WIDTH: u32 = 500;
const LANDSCAPE_TINY_IMAGE_WIDTH: u32 = 200;

const PORTRAIT_LARGE_IMAGE_WIDTH: u32 = 1500;
const PORTRAIT_SMALL_IMAGE_WIDTH: u32 = 300;
const PORTRAIT_TINY_IMAGE_WIDTH: u32 = 200;

const SQUARE_LARGE_IMAGE_WIDTH: u32 = 1500;
const SQUARE_SMALL_IMAGE_WIDTH: u32 = 400;
const SQUARE_TINY_IMAGE_WIDTH: u32 = 200;

impl From<(u32, u32)> for MediaDimensions {
    fn from(value: (u32, u32)) -> Self {
        MediaDimensions::new(value.0, value.1)
    }
}

pub enum ImageSize {
    Large,
    Small,
    Tiny,
}

impl ImageSize {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Large => "large",
            Self::Small => "small",
            Self::Tiny => "tiny",
        }
    }
}

pub enum ResizingConstraint {
    MaxWidth(u32),
}

impl ResizingConstraint {
    pub fn max_width(max_width: u32) -> ResizingConstraint {
        ResizingConstraint::MaxWidth(max_width)
    }
}

pub fn constrain_dimensions(
    original: &MediaDimensions,
    constraint: &ResizingConstraint,
) -> MediaDimensions {
    match constraint {
        ResizingConstraint::MaxWidth(max_width) => {
            if original.width <= *max_width {
                return *original;
            }

            let ratio = *max_width as f64 / original.width as f64;
            MediaDimensions::new(*max_width, (original.height as f64 * ratio) as u32)
        }
    }
}

fn resize_image_to_constraint(
    image: &DynamicImage,
    constraint: &ResizingConstraint,
) -> DynamicImage {
    let goal = constrain_dimensions(&image.dimensions().into(), constraint);

    image.resize(
        goal.width,
        goal.height,
        image::imageops::FilterType::Lanczos3,
    )
}

pub fn resize_image(image: &DynamicImage, size: &ImageSize) -> DynamicImage {
    let image_dimensions: MediaDimensions = image.dimensions().into();

    let target_width = match size {
        ImageSize::Large => match image_dimensions.orientation() {
            MediaOrientation::Landscape => LANDSCAPE_LARGE_IMAGE_WIDTH,
            MediaOrientation::Portrait => PORTRAIT_LARGE_IMAGE_WIDTH,
            MediaOrientation::Square => SQUARE_LARGE_IMAGE_WIDTH,
        },
        ImageSize::Small => match image_dimensions.orientation() {
            MediaOrientation::Landscape => LANDSCAPE_SMALL_IMAGE_WIDTH,
            MediaOrientation::Portrait => PORTRAIT_SMALL_IMAGE_WIDTH,
            MediaOrientation::Square => SQUARE_SMALL_IMAGE_WIDTH,
        },
        ImageSize::Tiny => match image_dimensions.orientation() {
            MediaOrientation::Landscape => LANDSCAPE_TINY_IMAGE_WIDTH,
            MediaOrientation::Portrait => PORTRAIT_TINY_IMAGE_WIDTH,
            MediaOrientation::Square => SQUARE_TINY_IMAGE_WIDTH,
        },
    };

    let constaint = ResizingConstraint::max_width(target_width);

    resize_image_to_constraint(image, &constaint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_original_dimensions_if_resizing_constraint_is_same_as_orignal() {
        let original = MediaDimensions::new(4000, 3000);
        let resized = constrain_dimensions(&original, &ResizingConstraint::MaxWidth(4000));

        assert_eq!(resized.width, 4000);
        assert_eq!(resized.height, 3000);
    }

    #[test]
    fn it_should_resize_image_to_within_max_width() {
        let original = MediaDimensions::new(4000, 3000);
        let resized = constrain_dimensions(&original, &ResizingConstraint::MaxWidth(2000));

        assert_eq!(resized.width, 2000);
        assert_eq!(resized.height, 1500);
    }

    #[test]
    fn it_should_preserve_aspect_ratio_when_resizing() {
        let original = MediaDimensions::new(4000, 3000);
        let resized = constrain_dimensions(&original, &ResizingConstraint::MaxWidth(2000));

        assert_eq!(resized.aspect_ratio(), 1.3333334);
    }

    #[test]
    fn it_should_not_upsize_image() {
        let original = MediaDimensions::new(2000, 1500);
        let resized = constrain_dimensions(&original, &ResizingConstraint::MaxWidth(4000));

        assert_eq!(resized.width, 2000);
        assert_eq!(resized.height, 1500);
    }
}
