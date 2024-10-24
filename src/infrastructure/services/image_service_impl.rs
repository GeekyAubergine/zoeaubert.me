use std::path::Path;

use chrono::{DateTime, Utc};
use imagesize::blob_size;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::domain::models::image::Image;
use crate::domain::models::slug::Slug;

use crate::domain::services::ImageService;
use crate::prelude::*;
use crate::{
    domain::{services::CacheService, services::CdnService, state::State},
    error::ImageError,
};

pub const MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

pub struct ImageServiceImpl;

impl ImageServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ImageService for ImageServiceImpl {
    async fn copy_image_from_url(
        &self,
        state: &impl State,
        url: &Url,
        path: &Path,
        alt: &str,
    ) -> Result<Image> {
        let (cache_path, data) = state
            .cache_service()
            .get_file_from_url(state, url)
            .await?;

        let image_size = blob_size(&data).map_err(ImageError::size_error)?;

        state.cdn_service().upload_file(state, &cache_path, path).await?;

        Ok(Image::new(
            path,
            alt,
            image_size.width as u32,
            image_size.height as u32,
        ))
    }

    async fn find_images_in_markdown(
        &self,
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

            let image = self
                .copy_image_from_url(state, &url, &path, alt)
                .await?
                .with_date(date)
                .with_parent_slug(parent_slug);

            media.push(image);
        }

        Ok(media)
    }
}
