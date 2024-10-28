use crate::domain::models::image::{Image, ImageDimensions};

pub enum ResizingConstraint {
    MaxWidth(u32),
}

impl ResizingConstraint {
    pub fn max_width(max_width: u32) -> ResizingConstraint {
        ResizingConstraint::MaxWidth(max_width)
    }
}

pub fn resize_image(
    original: &ImageDimensions,
    constraint: &ResizingConstraint,
) -> ImageDimensions {
    match constraint {
        ResizingConstraint::MaxWidth(max_width) => {
            if original.width <= *max_width {
                return *original;
            }

            let ratio = *max_width as f64 / original.width as f64;
            ImageDimensions::new(*max_width, (original.height as f64 * ratio) as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_original_dimensions_if_resizing_constraint_is_same_as_orignal() {
        let original = ImageDimensions::new(4000, 3000);
        let resized = resize_image(&original, &ResizingConstraint::MaxWidth(4000));

        assert_eq!(resized.width, 4000);
        assert_eq!(resized.height, 3000);
    }

    #[test]
    fn it_should_resize_image_to_within_max_width() {
        let original = ImageDimensions::new(4000, 3000);
        let resized = resize_image(&original, &ResizingConstraint::MaxWidth(2000));

        assert_eq!(resized.width, 2000);
        assert_eq!(resized.height, 1500);
    }

    #[test]
    fn it_should_preserve_aspect_ratio_when_resizing() {
        let original = ImageDimensions::new(4000, 3000);
        let resized = resize_image(&original, &ResizingConstraint::MaxWidth(2000));

        assert_eq!(resized.aspect_ratio(), 1.3333334);
    }

    #[test]
    fn it_should_not_upsize_image() {
        let original = ImageDimensions::new(2000, 1500);
        let resized = resize_image(&original, &ResizingConstraint::MaxWidth(4000));

        assert_eq!(resized.width, 2000);
        assert_eq!(resized.height, 1500);
    }
}
