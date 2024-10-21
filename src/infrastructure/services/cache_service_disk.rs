use std::path::Path;

use image::ImageError;
use imagesize::{blob_size, ImageSize};
use tracing::{debug, info};
use url::Url;

use crate::{
    domain::{models::cache_path::CachePath, services::CacheService},
    error::{FileSystemError, NetworkError},
    infrastructure::utils::{
        file_system::{make_cache_file_path, read_file, write_file},
        networking::download_bytes,
    },
};

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct CacheServiceDisk {
    reqwest_client: reqwest::Client,
}

impl CacheServiceDisk {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl CacheService for CacheServiceDisk {
    async fn is_file_cached(&self, path: &Path) -> Result<bool> {
        tokio::fs::metadata(path)
            .await
            .map_err(FileSystemError::read_error)
            .map(|_| true)
    }

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        read_file(&make_cache_file_path(path)).await
    }

    async fn write_file(&self, path: &Path, content: &[u8]) -> Result<()> {
        write_file(&make_cache_file_path(path), content).await
    }

    async fn download_and_cache_file(&self, url: &Url) -> Result<()> {
        self.get_file_from_cache_or_url(url).await?;

        Ok(())
    }

    async fn get_file_from_cache_or_url(&self, url: &Url) -> Result<Vec<u8>> {
        let url_path = url.path();

        let path = Path::new(&url_path);

        debug!("Getting file from cache or url: {} [{:?}]", url, path);

        if let Ok(content) = self.read_file(&path).await {
            return Ok(content);
        }

        let content = download_bytes(&self.reqwest_client, url).await?;

        self.write_file(&path, &content).await?;

        Ok(content)
    }
}
