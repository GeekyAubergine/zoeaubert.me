use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    application::jobs::albums::load_album_job::LoadAlbumJob, domain::models::{
        album::{Album, AlbumPhoto},
        media::image::Image,
        tag::Tag,
    }, error::Error, infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
        services::{cache::CachePath, date::parse_date, files::find_files_rescurse},
    }, prelude::Result
};

const ALBUMS_POSTS_DIR: &str = "albums";

#[derive(Debug, Clone, Deserialize)]
pub struct FileAlbumPhoto {
    url: String,
    description: String,
    alt: String,
    tags: Vec<String>,
    featured: Option<bool>,
}

impl FileAlbumPhoto {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn featured(&self) -> Option<bool> {
        self.featured
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileAlbum {
    title: String,
    description: Option<String>,
    date: String,
    photos: Vec<FileAlbumPhoto>,
}

impl FileAlbum {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn photos(&self) -> &[FileAlbumPhoto] {
        &self.photos
    }
}

#[derive(Debug)]
pub struct LoadAlbumsJob;

impl LoadAlbumsJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadAlbumsJob {
    fn name(&self) -> &str {
        "LoadAlbumsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading albums");

        let albums = find_files_rescurse(ALBUMS_POSTS_DIR, "yml", app_state.config())?;

        for album in albums {
            let album_content = app_state
                .content_dir()
                .read_file(&album, app_state.config())
                .await?;

            match serde_yaml::from_str(&album_content).map_err(Error::ParseAlbum) {
                Ok(album) => {
                    app_state.dispatch_job(LoadAlbumJob::new(album)).await?;
                }
                Err(e) => error!("Error parsing album: {:?}", e),
            }

            // let album_content = app_state
            //     .content_dir()
            //     .read_file(&album, app_state.config())
            //     .await?;

            // let album = album_file_to_album(app_state, &album_content).await?;
        }

        Ok(())
    }
}
