use std::path::Path;

use chrono::{DateTime, Utc};
use imagesize::{blob_size, ImageSize};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::error;
use url::Url;

use crate::domain::state::State;
use crate::prelude::*;

use crate::domain::models::image::Image;
use crate::domain::models::slug::Slug;
use crate::domain::services::CacheService;

use super::image_utils::image_from_url;

pub const MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

pub async fn extract_images_from_markdown(
    state: &impl State,
    markdown: &str,
    date: &DateTime<Utc>,
    parent_slug: &Slug,
) -> Result<Vec<Image>> {
    let mut media = vec![];

    for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
        let alt = cap.get(1).map_or("", |m| m.as_str());
        let url = cap.get(2).map_or("", |m| m.as_str());

        let url: Url = url.parse().unwrap();

        let path = url.path();

        let path = Path::new(&path);

        let image = image_from_url(state, &url, &path, alt)
            .await?
            .with_date(date)
            .with_parent_slug(parent_slug);

        media.push(image);
    }

    Ok(media)
}
