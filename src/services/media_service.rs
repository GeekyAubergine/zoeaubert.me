use std::io::Cursor;

use chrono::{DateTime, Utc};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{debug, info};
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
        file_service::{CacheFile, ReadableFile, WritableFile},
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

    fn read_image_size(ctx: &ServiceContext, file: &CacheFile) -> Result<MediaDimensions> {
        let byes = file.read()?;

        match imagesize::blob_size(&byes) {
            Ok(size) => Ok(MediaDimensions {
                width: size.width as u32,
                height: size.height as u32,
            }),
            Err(e) => Err(ImageError::size_error(e)),
        }
    }

    async fn resize_image(
        ctx: &ServiceContext,
        url: &Url,
        cdn_file: &CdnFile,
        original_image: &DynamicImage,
        size: &ImageSize,
    ) -> Result<SizedImage> {
        let file = cdn_file.as_cache_file();

        // If we already have it, don't bother processing
        if file.exists()? {
            let dimensions = Self::read_image_size(ctx, &file)?;

            return Ok(SizedImage {
                file: cdn_file.clone(),
                dimensions,
            });
        }

        let resized_image = resize_image(&original_image, size);

        let format = ImageFormat::from_path(&file.as_path_buff()).unwrap();

        let mut resized_image_data = vec![];
        resized_image
            .write_to(&mut Cursor::new(&mut resized_image_data), format)
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
        date: Option<DateTime<Utc>>,
    ) -> Result<Image> {
        let large_cdn_file =
            cdn_file.add_suffix_to_file_name(&format!("-{}", ImageSize::Large.as_str()));
        let small_cdn_file =
            cdn_file.add_suffix_to_file_name(&format!("-{}", ImageSize::Small.as_str()));
        let tiny_cdn_file =
            cdn_file.add_suffix_to_file_name(&format!("-{}", ImageSize::Tiny.as_str()));

        let original_file = cdn_file.as_cache_file();
        let large_file = large_cdn_file.as_cache_file();
        let small_file = small_cdn_file.as_cache_file();
        let tiny_file = tiny_cdn_file.as_cache_file();

        // If all exist, then don't process
        if original_file.exists()?
            && large_file.exists()?
            && small_file.exists()?
            && tiny_file.exists()?
        {
            debug!("Image already processed [{:?}]", &url.to_string());
            let original_size = Self::read_image_size(ctx, &original_file)?;
            let large_size = Self::read_image_size(ctx, &large_file)?;
            let small_size = Self::read_image_size(ctx, &small_file)?;
            let tiny_size = Self::read_image_size(ctx, &tiny_file)?;

            return Ok(Image {
                original: SizedImage {
                    file: cdn_file.clone(),
                    dimensions: original_size,
                },
                large: SizedImage {
                    file: large_cdn_file,
                    dimensions: large_size,
                },
                small: SizedImage {
                    file: small_cdn_file,
                    dimensions: small_size,
                },
                tiny: SizedImage {
                    file: tiny_cdn_file,
                    dimensions: tiny_size,
                },
                description: alt.to_string(),
                link_on_click: link_on_click.cloned(),
                date,
            });
        }

        info!("Processing image from URL [{:?}]", &url.to_string());

        let original_image = Self::read_or_download_image(ctx, url, cdn_file).await?;

        let large_image = Self::resize_image(
            ctx,
            url,
            &large_cdn_file,
            &original_image,
            &ImageSize::Large,
        )
        .await?;

        let small_image = Self::resize_image(
            ctx,
            url,
            &small_cdn_file,
            &original_image,
            &ImageSize::Small,
        )
        .await?;

        let tiny_image =
            Self::resize_image(ctx, url, &tiny_cdn_file, &original_image, &ImageSize::Tiny).await?;

        Ok(Image {
            original: SizedImage {
                file: cdn_file.clone(),
                dimensions: original_image.dimensions().into(),
            },
            large: large_image,
            small: small_image,
            tiny: tiny_image,
            description: alt.to_string(),
            link_on_click: link_on_click.cloned(),
            date,
        })
    }

    pub async fn find_images_in_markdown(
        ctx: &ServiceContext,
        markdown: &str,
        date: Option<DateTime<Utc>>,
        link_on_click: Option<&Slug>,
    ) -> Result<Vec<Image>> {
        let mut media = vec![];

        for cap in MARKDOWN_IMAGE_REGEX.captures_iter(markdown) {
            let alt = cap.get(1).map_or("", |m| m.as_str());
            let url = cap.get(2).map_or("", |m| m.as_str());

            let url: Url = url.parse().unwrap();
            let cdn_file = CdnFile::from_str(url.path());

            let image = Self::image_from_url(ctx, &url, &cdn_file, alt, link_on_click, date).await?;

            media.push(image);
        }

        Ok(media)
    }
}
