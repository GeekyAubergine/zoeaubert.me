use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::info;

use crate::{
    application::jobs::cdn::resize_and_upload_image_to_cdn_job::ResizeAndUploadImageToCdnJob,
    domain::models::{
        album::{Album, AlbumPhoto},
        media::image::{Image, ImageOrientation},
        tag::Tag,
    },
    error::Error,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::{Job, JobPriority},
        config::Config,
        services::{cache::CachePath, date::parse_date, files::find_files_rescurse},
    },
    prelude::Result,
};

use super::load_albums_job::FileAlbum;

const LANDSCAPE_SMALL_IMAGE_WIDTH: u32 = 500;
const LANDSCAPE_LARGE_IMAGE_WIDTH: u32 = 2000;

const PORTRAIT_SMALL_IMAGE_WIDTH: u32 = 300;
const PORTRAIT_LARGE_IMAGE_WIDTH: u32 = 1500;

const SQUARE_SMALL_IMAGE_WIDTH: u32 = 400;
const SQUARE_LARGE_IMAGE_WIDTH: u32 = 1500;

fn image_size_resized_small(image: &Image) -> (u32, u32) {
    match image.orientation() {
        ImageOrientation::Landscape => (
            LANDSCAPE_SMALL_IMAGE_WIDTH,
            (image.aspect_ratio() * LANDSCAPE_SMALL_IMAGE_WIDTH as f32) as u32,
        ),
        ImageOrientation::Portrait => (
            PORTRAIT_SMALL_IMAGE_WIDTH,
            (image.aspect_ratio() * PORTRAIT_SMALL_IMAGE_WIDTH as f32) as u32,
        ),
        ImageOrientation::Square => (
            SQUARE_SMALL_IMAGE_WIDTH,
            (image.aspect_ratio() * SQUARE_SMALL_IMAGE_WIDTH as f32) as u32,
        ),
    }
}

fn image_size_resized_large(image: &Image) -> (u32, u32) {
    match image.orientation() {
        ImageOrientation::Landscape => (
            LANDSCAPE_LARGE_IMAGE_WIDTH,
            image.aspect_ratio() as u32 * LANDSCAPE_LARGE_IMAGE_WIDTH,
        ),
        ImageOrientation::Portrait => (
            PORTRAIT_LARGE_IMAGE_WIDTH,
            image.aspect_ratio() as u32 * PORTRAIT_LARGE_IMAGE_WIDTH,
        ),
        ImageOrientation::Square => (
            SQUARE_LARGE_IMAGE_WIDTH,
            image.aspect_ratio() as u32 * SQUARE_LARGE_IMAGE_WIDTH,
        ),
    }
}

fn add_size_suffix_to_cache_path(path: &CachePath, suffix: &str, config: &Config) -> CachePath {
    let path = path.path().to_owned();

    let mut parts = path.split('.').collect::<Vec<&str>>();
    let extension = parts.pop().unwrap();

    let path = path.replace(&format!(".{}", extension), "");

    let path = format!("{}{}.{}", path, suffix, extension);

    CachePath::new(config, path)
}

async fn album_file_to_album(app_state: &AppState, file_album: &FileAlbum) -> Result<Album> {
    let mut album = Album::new(
        file_album.title().to_owned(),
        file_album.description().map(|s| s.to_owned()),
        parse_date(file_album.date())?,
    );

    for photo in file_album.photos() {
        let tags = photo.tags().iter().map(|t| Tag::from_string(t)).collect();

        let featured = photo.featured().unwrap_or(false);

        let cache_path = CachePath::new(app_state.config(), photo.url().to_string());

        let image_size = app_state
            .cache()
            .get_image_size_from_cache_or_download(app_state, &cache_path)
            .await?;

        let file_name = photo.url().split('/').last().unwrap();
        let file_name_without_extension = file_name.split('.').next().unwrap();

        let original_image = Image::new(
            &cache_path
                .cdn_path(app_state.config())
                .url(app_state.config()),
            photo.alt(),
            image_size.width as u32,
            image_size.height as u32,
        )
        .with_date(*album.date())
        .with_description(photo.description())
        .with_parent_permalink(&format!("{}/{}", album.permalink(), file_name));

        let small_cache_path =
            add_size_suffix_to_cache_path(&cache_path, "-small", app_state.config());

        let (small_width, small_height) = image_size_resized_small(&original_image);

        let small_image = Image::new(
            &small_cache_path
                .cdn_path(app_state.config())
                .url(app_state.config()),
            photo.alt(),
            small_width,
            small_height,
        )
        .with_date(*album.date())
        .with_description(photo.description())
        .with_parent_permalink(&format!("{}{}", album.permalink(), file_name));

        let large_cache_path =
            add_size_suffix_to_cache_path(&cache_path, "-large", app_state.config());

        let (large_width, large_height) = image_size_resized_large(&original_image);

        let large_image = Image::new(
            &large_cache_path
                .cdn_path(app_state.config())
                .url(app_state.config()),
            photo.alt(),
            large_width,
            large_height,
        )
        .with_date(*album.date())
        .with_description(photo.description())
        .with_parent_permalink(&format!("{}{}", album.permalink(), file_name));

        let photo = AlbumPhoto::new(
            small_image.clone(),
            large_image.clone(),
            original_image.clone(),
            file_name.to_string(),
            tags,
            featured,
        );

        album.add_photo(photo);

        app_state
            .dispatch_job(
                ResizeAndUploadImageToCdnJob::new(
                    cache_path.clone(),
                    small_width,
                    small_height,
                    small_cache_path,
                ),
                JobPriority::Low,
            )
            .await?;

        app_state
            .dispatch_job(
                ResizeAndUploadImageToCdnJob::new(
                    cache_path,
                    large_width,
                    large_height,
                    large_cache_path,
                ),
                JobPriority::Low,
            )
            .await?;
    }

    Ok(album)
}

#[derive(Debug)]
pub struct LoadAlbumJob {
    file_album: FileAlbum,
}

impl LoadAlbumJob {
    pub fn new(file_album: FileAlbum) -> Self {
        Self { file_album }
    }
}

#[async_trait]
impl Job for LoadAlbumJob {
    fn name(&self) -> &str {
        "LoadAlbumsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let album = album_file_to_album(app_state, &self.file_album).await?;

        app_state.albums_repo().commit(album).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_size_suffix_to_cache_path() {
        let config = Config::default();

        let path = CachePath::new(&config, "path/to/image.jpg".to_string());

        let path = add_size_suffix_to_cache_path(&path, "-small", &config);

        let expected = CachePath::new(&config, "path/to/image-small.jpg".to_string());

        assert_eq!(path, expected);
    }
}
