use std::io::Cursor;

use chrono::{DateTime, Utc};
use image::{DynamicImage, GenericImageView, ImageReader};
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::{
    domain::models::{
        image::{Image, SizedImage},
        media::{MediaDimensions, MediaOrientation},
        slug::Slug,
    },
    error::ImageError,
    prelude::*,
    services::{
        cdn_service::CdnFile,
        file_service::{ReadableFile, WritableFile},
        ServiceContext,
    },
    utils::resize_image::{resize_image, ImageSize},
};
pub const MARKDOWN_IMAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)!\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

pub struct MediaService;

impl MediaService {
    async fn read_or_download_file(
        ctx: &ServiceContext,
        url: &Url,
        cdn_path: &CdnFile,
    ) -> Result<Vec<u8>> {
        let file = cdn_path.as_cache_file();

        if file.exists()? {
            return file.read();
        }

        let data = ctx.network.download_bytes(url).await?;

        file.write(&data)?;

        Ok(data)
    }

    async fn read_or_download_image(
        ctx: &ServiceContext,
        url: &Url,
        cdn_file: &CdnFile,
    ) -> Result<DynamicImage> {
        let original_bytes = Self::read_or_download_file(ctx, url, &cdn_file).await?;

        let original_image = ImageReader::new(Cursor::new(&original_bytes))
            .with_guessed_format()
            .map_err(ImageError::parse_format_error)?
            .decode()
            .map_err(ImageError::decode_error)?;

        Ok(original_image)
    }

    async fn resize_image(
        ctx: &ServiceContext,
        url: &Url,
        cdn_file: &CdnFile,
        original_image: &DynamicImage,
        size: &ImageSize,
    ) -> Result<SizedImage> {
        let cdn_file = cdn_file.add_suffix_to_file_name(&format!("-{}", size.as_str()));

        let file = cdn_file.as_cache_file();

        // If we already have it, don't bother processing
        if file.exists()? {
            let image = Self::read_or_download_image(ctx, url, &cdn_file).await?;

            return Ok(SizedImage {
                file: cdn_file.clone(),
                dimensions: image.dimensions().into(),
            });
        }

        let resized_image = resize_image(&original_image, size);

        let mut resized_image_data = vec![];
        resized_image
            .write_to(
                &mut Cursor::new(&mut resized_image_data),
                image::ImageFormat::Jpeg,
            )
            .map_err(ImageError::encode_error)?;

        file.write(&resized_image_data)?;

        ctx.cdn.upload_file(ctx, &file, &cdn_file).await?;

        Ok(SizedImage {
            file: cdn_file.clone(),
            dimensions: original_image.dimensions().into(),
        })
    }

    pub async fn image_from_url(
        ctx: &ServiceContext,
        url: &Url,
        cdn_file: &CdnFile,
        alt: &str,
        link_on_click: Option<&Slug>,
    ) -> Result<Image> {
        let original_image = Self::read_or_download_image(ctx, url, cdn_file).await?;

        let small_image =
            Self::resize_image(ctx, url, cdn_file, &original_image, &ImageSize::Small).await?;
        let large_image =
            Self::resize_image(ctx, url, cdn_file, &original_image, &ImageSize::Large).await?;

        Ok(Image {
            original: SizedImage {
                file: cdn_file.clone(),
                dimensions: original_image.dimensions().into(),
            },
            large: large_image,
            small: small_image,
            description: alt.to_string(),
            link_on_click: link_on_click.cloned(),
        })
    }

    pub async fn find_images_in_markdown(
        ctx: &ServiceContext,
        markdown: &str,
        date: &DateTime<Utc>,
        link_on_click: Option<&Slug>,
    ) -> Result<Vec<Image>> {
        let mut media = vec![];

        for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
            let alt = cap.get(1).map_or("", |m| m.as_str());
            let url = cap.get(2).map_or("", |m| m.as_str());

            let url: Url = url.parse().unwrap();
            let cdn_file = CdnFile::from_str(url.path());

            let image = Self::image_from_url(ctx, &url, &cdn_file, alt, link_on_click).await?;

            media.push(image);
        }

        Ok(media)
    }
}
