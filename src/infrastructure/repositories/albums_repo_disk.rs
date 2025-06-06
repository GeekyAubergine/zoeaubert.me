use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Datelike;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::albums::album::Album;
use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::AlbumsRepo;
use crate::domain::services::FileService;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

const FILE_NAME: &str = "albums_posts.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AlbumsRepoData {
    albums: HashMap<Slug, Album>,
}

impl AlbumsRepoData {
    pub fn new() -> Self {
        Self {
            albums: HashMap::new(),
        }
    }
}

pub struct AlbumsRepoDisk {
    data: Arc<RwLock<AlbumsRepoData>>,
    file_service: FileServiceDisk,
}

impl AlbumsRepoDisk {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl AlbumsRepo for AlbumsRepoDisk {
    async fn find_all_by_date(&self) -> Result<Vec<Album>> {
        let data = self.data.read().await;

        let mut albums = data.albums.values().cloned().collect::<Vec<Album>>();

        albums.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(albums)
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Album>> {
        let data = self.data.read().await;
        Ok(data.albums.get(slug).cloned())
    }

    async fn find_grouped_by_year(&self) -> Result<Vec<(u16, Vec<Album>)>> {
        let years: HashMap<u16, Vec<Album>> = self
            .data
            .read()
            .await
            .albums
            .values()
            .cloned()
            .into_iter()
            .fold(HashMap::new(), |mut acc, album| {
                acc.entry(album.date.year() as u16)
                    .or_insert_with(Vec::new)
                    .push(album);
                acc
            });

        let mut years = years.into_iter().collect::<Vec<(u16, Vec<Album>)>>();

        for (_, albums) in &mut years {
            albums.sort_by(|a, b| b.date.cmp(&a.date));
        }

        years.sort_by(|a, b| b.0.cmp(&a.0));

        Ok(years)
    }

    async fn find_all_album_photos(&self) -> Result<Vec<AlbumPhoto>> {
        let data = self.data.read().await;
        let mut photos = Vec::new();
        for album in data.albums.values() {
            photos.extend(album.photos.clone());
        }
        Ok(photos)
    }

    async fn commit(&self, album: &Album) -> Result<()> {
        let mut data = self.data.write().await;
        data.albums.insert(album.slug.clone(), album.clone());
        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &*data)
            .await?;
        Ok(())
    }
}
