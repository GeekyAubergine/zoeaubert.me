use chrono::{DateTime, Utc};
use imagesize::{blob_size, ImageSize};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::error;

use crate::domain::models::image::Image;
use crate::domain::models::slug::Slug;
use crate::domain::services::CacheService;

pub const HTML_IMAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)<img(((src="(?<src>([^"]+))")|(alt="(?<alt>([^"]+))")|(width="(?<width>([^"]+))")|(height="(?<height>([^"]+))"))|[^>])*>"#).unwrap()
});
pub const MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

pub fn extract_images_from_html(
    markdown: &str,
    date: &DateTime<Utc>,
    parent_slug: &Slug,
) -> Vec<Image> {
    let mut images = vec![];

    for cap in HTML_IMAGE_REGEX.captures_iter(markdown) {
        let url = cap.name("src").map_or("", |m| m.as_str());
        let alt = cap.name("alt").map_or("", |m| m.as_str());
        let width = cap.name("width").map_or("", |m| m.as_str());
        let height = cap.name("height").map_or("", |m| m.as_str());

        let width = width.parse::<u32>().unwrap_or(0);
        let height = height.parse::<u32>().unwrap_or(0);

        images.push(
            Image::new(url, alt, width, height)
                .with_date(date)
                .with_parent_slug(parent_slug),
        );
    }

    images
}

pub async fn extract_images_from_markdown(
    markdown: &str,
    cache: &impl CacheService,
    date: &DateTime<Utc>,
    parent_slug: &Slug,
) -> Vec<Image> {
    let mut media = vec![];

    for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
        let alt = cap.get(1).map_or("", |m| m.as_str());
        let url = cap.get(2).map_or("", |m| m.as_str());

        let file = cache.get_file_from_cache_or_url(url).await;

        let file = match file {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to get file for {}: {:?}", url, e);
                continue;
            }
        };

        let image_size = match blob_size(&file) {
            Ok(size) => size,
            Err(e) => {
                error!("Failed to get image size for {}: {:?}", url, e);
                ImageSize {
                    width: 0,
                    height: 0,
                }
            }
        };

        if image_size.width == 0 || image_size.height == 0 {
            continue;
        }

        media.push(
            Image::new(url, alt, image_size.width as u32, image_size.height as u32)
                .with_date(date)
                .with_parent_slug(parent_slug),
        );
    }

    media
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_media_from_html() {
        let parent_slug = Slug::new("test-slug");

        let markdown = r#"Movie friend\n\n<img src="uploads/2022/ced7ff5352.jpg" width="600" height="450" alt="Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa">\n"#;

        let date = Utc::now();

        let media = extract_images_from_html(markdown, &date, &parent_slug);

        assert_eq!(media.len(), 1);

        let expected_uuid = uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_URL,
            "uploads/2022/ced7ff5352.jpg".as_bytes(),
        );

        let expected = Image::new(
            "uploads/2022/ced7ff5352.jpg",
            "Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa",
            600,
             450,
        ).with_date(&date).with_parent_slug(&parent_slug);

        let image = media.get(0).unwrap();

        assert_eq!(image.uuid, expected.uuid);
        assert_eq!(image.url, expected.url);
        assert_eq!(image.alt, expected.alt);
        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
    }
}
