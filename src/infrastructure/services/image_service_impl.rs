use std::path::Path;

use chrono::{DateTime, Utc};
use image::ImageReader;
use imagesize::blob_size;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::Cursor;
use url::Url;

use crate::domain::models::image::{Image};
use crate::domain::models::media::MediaDimensions;
use crate::domain::models::slug::Slug;

use crate::domain::services::{ImageService, NetworkService};
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
        let data = state.network_service().download_bytes(&url).await?;

        state
            .cache_service()
            .write_file(state, &path, &data)
            .await?;

        let image_size = blob_size(&data).map_err(ImageError::size_error)?;

        state.cdn_service().upload_file(state, &path, path).await?;

        Ok(Image::new(
            path,
            alt,
            &MediaDimensions::new(image_size.width as u32, image_size.height as u32),
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

    async fn copy_and_resize_image_from_url(
        &self,
        state: &impl State,
        url: &Url,
        path: &Path,
        alt: &str,
        new_size: &MediaDimensions,
    ) -> Result<Image> {
        let original_bytes = state.network_service().download_bytes(&url).await?;

        state
            .cache_service()
            .write_file(state, &path, &original_bytes)
            .await?;

        let image = ImageReader::new(Cursor::new(&original_bytes))
            .with_guessed_format()
            .map_err(ImageError::parse_format_error)?
            .decode()
            .map_err(ImageError::decode_error)?;

        let resized = image.resize(
            new_size.width,
            new_size.height,
            image::imageops::FilterType::Lanczos3,
        );

        let url_path = url.path();

        let cache_path = Path::new(&url_path);

        let mut resized_image_data = vec![];
        resized
            .write_to(
                &mut Cursor::new(&mut resized_image_data),
                image::ImageFormat::Jpeg,
            )
            .map_err(ImageError::encode_error)?;

        state
            .cache_service()
            .write_file(state, &path, &resized_image_data)
            .await?;

        state
            .cdn_service()
            .upload_file(state, &path, path)
            .await?;

        Ok(Image::new(path, alt, new_size))
    }
}
