use std::{io::Cursor, path::Path};

use chrono::{DateTime, Utc};
use image::ImageReader;
use imagesize::blob_size;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::domain::models::slug::Slug;
use crate::prelude::*;

use crate::services::cdn_service::CdnService;
use crate::services::file_service::FilePath;
use crate::services::ServiceContext;
use crate::{
    domain::models::{image::Image, media::MediaDimensions},
    error::ImageError,
    services::network_service::NetworkService2,
};

pub const MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

pub struct ImageService2;

impl ImageService2 {
    pub fn new() -> Self {
        Self
    }

    pub async fn copy_image_from_url(
        &self,
        ctx: &ServiceContext,
        url: &Url,
        path: &FilePath,
        alt: &str,
    ) -> Result<Image> {
        let data = ctx.network.download_bytes(&url).await?;

        path.write(&data).await?;

        let image_size = blob_size(&data).map_err(ImageError::size_error)?;

        ctx.cdn.upload_file(ctx, &path, path).await?;

        Ok(Image::new(
            path,
            alt,
            &MediaDimensions::new(image_size.width as u32, image_size.height as u32),
        ))
    }

    pub async fn find_images_in_markdown(
        &self,
        ctx: &ServiceContext,
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

            let path = FilePath::cache(path);

            let image = self
                .copy_image_from_url(ctx, &url, &path, alt)
                .await?
                .with_date(date)
                .with_parent_slug(parent_slug);

            media.push(image);
        }

        Ok(media)
    }

    pub async fn copy_and_resize_image_from_url(
        &self,
        ctx: &ServiceContext,
        url: &Url,
        path: &FilePath,
        alt: &str,
        new_size: &MediaDimensions,
    ) -> Result<Image> {
        let original_bytes = ctx.network.download_bytes(&url).await?;

        path.write(&original_bytes).await?;

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

        path.write(&resized_image_data).await?;

        ctx.cdn.upload_file(ctx, &path, path).await?;

        Ok(Image::new(path, alt, new_size))
    }
}
