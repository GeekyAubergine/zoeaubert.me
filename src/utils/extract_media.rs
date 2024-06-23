use std::os::unix::process;

use chrono::{DateTime, Utc};
use regex::Regex;
use tracing::error;

use crate::{
    domain::models::media::{image::Image, Media},
    infrastructure::app_state::{self, AppState},
};

lazy_static! {
    pub static ref HTML_IMAGE_REGEX: Regex = Regex::new(r#"(?i)<img[^>]*?src="([^"]+)"[^>]*alt="([^"]+)"[^>]*width="([^"]+)"[^>]*height="([^"]+)"[^>]*>"#).unwrap();
    pub static ref MARKDOWN_IMAGE_REGEX: Regex = Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap();
}

pub fn extract_media_from_html(markdown: &str) -> Vec<Media> {
    let mut media = vec![];

    for cap in HTML_IMAGE_REGEX.captures_iter(markdown) {
        let url = cap.get(1).map_or("", |m| m.as_str());
        let alt = cap.get(2).map_or("", |m| m.as_str());
        let width = cap.get(3).map_or("", |m| m.as_str());
        let height = cap.get(4).map_or("", |m| m.as_str());

        let width = width.parse::<u32>().unwrap_or(0);
        let height = height.parse::<u32>().unwrap_or(0);

        media.push(Media::from_image(Image::new(url, alt, width, height)));
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

        // Trim domain
        let path = url.split('/').skip(3).collect::<Vec<&str>>().join("/");

        match app_state
            .cache()
            .get_image_size_from_cache_or_download(app_state, &path)
            .await
        {
            Ok(size) => {
                media.push(Media::from_image(
                    Image::new(url, alt, size.width as u32, size.height as u32)
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
        let markdown = r#"This is a bunch of stuff before <img src="https://cdn.geekyaubergine.com/uploads/2021/08/2021-08-01-09-00-00-0001.jpeg" alt="Image" width="100" height="100"> somet thigns arsdlfkjsd"#;

        let media = extract_media_from_html(markdown);

        assert_eq!(media.len(), 1);

        let expected = Media::from_image(Image::new(
            "https://cdn.geekyaubergine.com/uploads/2021/08/2021-08-01-09-00-00-0001.jpeg",
            "Image",
            100,
            100,
        ));

        assert_eq!(media[0], expected);
    }
}
