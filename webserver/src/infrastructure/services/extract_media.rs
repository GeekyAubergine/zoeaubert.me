use std::os::unix::process;

use chrono::{DateTime, Utc};
use regex::Regex;
use tracing::error;

use crate::{
    domain::models::media::{image::Image, Media},
    infrastructure::app_state::{self, AppState},
};

use super::cache::CachePath;

lazy_static! {
    pub static ref HTML_IMAGE_REGEX: Regex = Regex::new(r#"(?i)<img(((src="(?<src>([^"]+))")|(alt="(?<alt>([^"]+))")|(width="(?<width>([^"]+))")|(height="(?<height>([^"]+))"))|[^>])*>"#).unwrap();
    pub static ref MARKDOWN_IMAGE_REGEX: Regex = Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap();
}

pub fn extract_media_from_html(
    markdown: &str,
    parent_permalink: &str,
    date: &DateTime<Utc>,
) -> Vec<Media> {
    let mut media = vec![];

    for cap in HTML_IMAGE_REGEX.captures_iter(markdown) {
        let url = cap.name("src").map_or("", |m| m.as_str());
        let alt = cap.name("alt").map_or("", |m| m.as_str());
        let width = cap.name("width").map_or("", |m| m.as_str());
        let height = cap.name("height").map_or("", |m| m.as_str());

        let uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, url.as_bytes());

        let width = width.parse::<u32>().unwrap_or(0);
        let height = height.parse::<u32>().unwrap_or(0);

        media.push(Media::from_image(
            Image::new(&uuid, url, alt, width, height)
                .with_date(date.to_owned())
                .with_parent_permalink(parent_permalink),
        ));
    }

    media
}

pub async fn extract_media_from_markdown(
    app_state: &AppState,
    markdown: &str,
    parent_permalink: &str,
    date: &DateTime<Utc>,
) -> Vec<Media> {
    let mut media = vec![];

    for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
        let alt = cap.get(1).map_or("", |m| m.as_str());
        let url = cap.get(2).map_or("", |m| m.as_str());

        let uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, url.as_bytes());

        match app_state
            .cache()
            .get_image_size_from_cache_or_download(
                app_state,
                &CachePath::from_url(app_state.config(), url),
            )
            .await
        {
            Ok(size) => {
                media.push(Media::from_image(
                    Image::new(&uuid, url, alt, size.width as u32, size.height as u32)
                        .with_parent_permalink(parent_permalink)
                        .with_date(date.to_owned()),
                ));
            }
            Err(e) => {
                error!("Failed to get image size for {}: {:?}", url, e);
            }
        }
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

        let media = extract_media_from_html(markdown, "perma", &date);

        assert_eq!(media.len(), 1);

        let expected_uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, "uploads/2022/ced7ff5352.jpg".as_bytes());

        let expected = Image::new(
            &expected_uuid,
            "uploads/2022/ced7ff5352.jpg",
            "Pciture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa",
            600,
            450,
        ).with_parent_permalink("perma");

        let image = match media.get(0).unwrap() {
            Media::Image(image) => image,
            _ => panic!("Expected image"),
        };

        assert_eq!(image.uuid(), expected.uuid());
        assert_eq!(image.url(), expected.url());
        assert_eq!(image.alt(), expected.alt());
        assert_eq!(image.width(), expected.width());
        assert_eq!(image.height(), expected.height());
        assert_eq!(image.parent_permalink(), expected.parent_permalink());
    }
}
