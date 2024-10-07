use chrono::{DateTime, Utc};
use regex::Regex;
use shared::{cache::CacheService, zoeaubert_proto::webserver::Image};
use tracing::error;
use uuid::Uuid;

use imagesize::{blob_size, ImageSize};

lazy_static! {
    pub static ref HTML_IMAGE_REGEX: Regex = Regex::new(r#"(?i)<img(((src="(?<src>([^"]+))")|(alt="(?<alt>([^"]+))")|(width="(?<width>([^"]+))")|(height="(?<height>([^"]+))"))|[^>])*>"#).unwrap();
    pub static ref MARKDOWN_IMAGE_REGEX: Regex = Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap();
}

pub fn extract_images_from_html(markdown: &str, date: &DateTime<Utc>) -> Vec<Image> {
    let mut images = vec![];

    for cap in HTML_IMAGE_REGEX.captures_iter(markdown) {
        let url = cap.name("src").map_or("", |m| m.as_str());
        let alt = cap.name("alt").map_or("", |m| m.as_str());
        let width = cap.name("width").map_or("", |m| m.as_str());
        let height = cap.name("height").map_or("", |m| m.as_str());

        let width = width.parse::<i32>().unwrap_or(0);
        let height = height.parse::<i32>().unwrap_or(0);

        images.push(Image {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes()).to_string(),
            url: url.to_string(),
            alt: alt.to_string(),
            width,
            height,
            date: date.to_rfc3339(),
        });
    }

    images
}

pub async fn extract_media_from_markdown(
    markdown: &str,
    date: &DateTime<Utc>,
    cache: &dyn CacheService,
) -> Vec<Image> {
    let mut media = vec![];

    for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
        let alt = cap.get(1).map_or("", |m| m.as_str());
        let url = cap.get(2).map_or("", |m| m.as_str());

        let uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, url.as_bytes());

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

        media.push(Image {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes()).to_string(),
            url: url.to_string(),
            alt: alt.to_string(),
            width: image_size.width as i32,
            height: image_size.height as i32,
            date: date.to_rfc3339(),
        });
    }

    media
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_media_from_html() {
        let markdown = r#"Movie friend\n\n<img src="uploads/2022/ced7ff5352.jpg" width="600" height="450" alt="Pciture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa">\n"#;

        let date = Utc::now();

        let media = extract_images_from_html(markdown, &date);

        assert_eq!(media.len(), 1);

        let expected_uuid = uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_URL,
            "uploads/2022/ced7ff5352.jpg".as_bytes(),
        );

        let expected = Image {
            uuid: expected_uuid.to_string(),
            url: "uploads/2022/ced7ff5352.jpg".to_string(),
            alt: "Pciture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa".to_string(),
            width: 600,
            height: 450,
            date: date.to_rfc3339(),
        };

        let image = media.get(0).unwrap();

        assert_eq!(image.uuid, expected.uuid);
        assert_eq!(image.url, expected.url);
        assert_eq!(image.alt, expected.alt);
        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
    }
}
