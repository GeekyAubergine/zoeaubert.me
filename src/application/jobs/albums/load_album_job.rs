use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::models::{
        album::{Album, AlbumPhoto},
        media::image::Image,
        tag::Tag,
    },
    error::Error,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
        services::{cache::CachePath, date::parse_date, files::find_files_rescurse},
    },
    prelude::Result,
};

use super::load_albums_job::FileAlbum;

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

        let image = Image::new(
            &format!("{}{}", app_state.config().cdn_url(), photo.url()),
            photo.alt(),
            image_size.width as u32,
            image_size.height as u32,
        )
        .with_date(*album.date())
        .with_description(photo.description())
        .with_parent_permalink(&format!("{}/{}", album.permalink(), file_name));

        let photo = AlbumPhoto::new(
            image.clone(),
            image.clone(),
            image.clone(),
            file_name.to_string(),
            tags,
            featured,
        );

        album.add_photo(photo);
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
