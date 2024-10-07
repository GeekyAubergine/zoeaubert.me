use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use shared::utils::date::parse_date;
use tracing::info;
use uuid::Uuid;

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
        services::{cache::CachePath,files::find_files_rescurse},
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

const ALBUM_UUID_SEED: Uuid = Uuid::from_u128((0x4be2bc8577962c625592ca3d9d4a12db));
const ALBUM_PHOTO_UUID_SEED: Uuid = Uuid::from_u128((0xe2c3009aa37304d49ff03faf8a30a5d5));
const ALBUM_PHOTO_ORIGINAL_UUID_SEED: Uuid = Uuid::from_u128((0x5220167f6021ab45b96d5169ded097a4));
const ALBUM_PHOTO_SMALL_UUID_SEED: Uuid = Uuid::from_u128((0xb63c484040de59f873fadbac1aa8880c));
const ALBUM_PHOTO_LARGE_UUID_SEED: Uuid = Uuid::from_u128((0x223d3fb11524190f0e654d7c27794c4e));

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
    let album_title = file_album.title();
    let album_description = file_album.description().map(|s| s.to_owned());
    let album_date = parse_date(file_album.date())?;

    let album_key = format!("{}{}", album_title, album_date);

    let mut album = Album::new(
        &Uuid::new_v5(&ALBUM_UUID_SEED, album_key.as_bytes()),
        album_title.to_string(),
        album_description,
        album_date,
    );

    for photo in file_album.photos() {
        let tags = photo.tags().iter().map(|t| Tag::from_string(t)).collect::<Vec<Tag>>();

        let featured = photo.featured().unwrap_or(false);

        let cache_path = CachePath::new(app_state.config(), photo.url().to_string());

        let image_size = app_state
            .cache()
            .get_image_size_from_cache_or_download(app_state, &cache_path)
            .await?;

        let file_name = photo.url().split('/').last().unwrap();
        let file_name_without_extension = file_name.split('.').next().unwrap();

        let original_image = Image::new(
            &Uuid::new_v5(
                &ALBUM_PHOTO_ORIGINAL_UUID_SEED,
                cache_path.path().as_bytes(),
            ),
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
            &Uuid::new_v5(
                &ALBUM_PHOTO_SMALL_UUID_SEED,
                small_cache_path.path().as_bytes(),
            ),
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
            &Uuid::new_v5(
                &ALBUM_PHOTO_LARGE_UUID_SEED,
                large_cache_path.path().as_bytes(),
            ),
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

        let album_photo_key = format!("{}{}", album_key, file_name_without_extension);

        let photo = AlbumPhoto::new(
            Uuid::new_v5(&ALBUM_PHOTO_UUID_SEED, album_photo_key.as_bytes()),
            album.uuid().clone(),
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
