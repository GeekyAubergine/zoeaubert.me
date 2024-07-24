use std::io::Cursor;

use async_trait::async_trait;
use image::ImageReader;
use tracing::{debug, info};

use crate::{
    error::Error,
    infrastructure::{app_state::AppState, bus::job_runner::Job, services::cache::CachePath},
    prelude::Result,
};

#[derive(Debug)]
pub struct ResizeAndUploadImageToCdnJob {
    cache_path: CachePath,
    width: u32,
    height: u32,
    new_cache_path: CachePath,
}

impl ResizeAndUploadImageToCdnJob {
    pub fn new(cache_path: CachePath, width: u32, height: u32, new_cache_path: CachePath) -> Self {
        Self {
            cache_path,
            width,
            height,
            new_cache_path,
        }
    }
}

#[async_trait]
impl Job for ResizeAndUploadImageToCdnJob {
    fn name(&self) -> &str {
        "ResizeAndUploadImageToCdnJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let cnd_path = self.cache_path.cdn_path(app_state.config());
        let new_cdn_path = self.new_cache_path.cdn_path(app_state.config());

        if app_state
            .cdn()
            .file_exists(&new_cdn_path, app_state.config())
            .await?
        {
            debug!(
                "Resized image already exists in CDN [{}], skipping",
                new_cdn_path
            );
            return Ok(());
        }

        println!("{:?}", self);

        info!(
            "Resizing and uploading image from CDN [{}] to CDN [{}]",
            cnd_path, new_cdn_path
        );

        let original_image_data = app_state
            .cache()
            .get_file_from_cache_or_download(
                app_state,
                &self.cache_path,
                &cnd_path.url(app_state.config()),
            )
            .await?;

        let original_image = ImageReader::new(Cursor::new(original_image_data))
            .with_guessed_format()
            .map_err(Error::UnableToParseImageFormat)?
            .decode()
            .map_err(Error::UnableToDecodeImage)?;

        let resized_image = original_image.resize(
            self.width,
            self.height,
            image::imageops::FilterType::Lanczos3,
        );

        let mut resized_image_data = Vec::new();
        resized_image
            .write_to(
                &mut Cursor::new(&mut resized_image_data),
                image::ImageFormat::Jpeg,
            )
            .map_err(Error::UnableToEncodeImage)?;

        app_state
            .cache()
            .cache_file(&self.new_cache_path, resized_image_data.clone())
            .await?;

        app_state
            .cdn()
            .upload_file_from_bytes(resized_image_data, &new_cdn_path, app_state.config())
            .await?;

        Ok(())
    }
}
