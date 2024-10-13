use tracing::{debug, info};

use crate::{
    domain::{models::cache_path::CachePath, services::CacheService},
    error::{FileSystemError, NetworkError},
    infrastructure::utils::{file_system::{make_cache_file_path, read_file, write_file}, networking::download_bytes},
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
    async fn is_file_cached(&self, path: &CachePath) -> Result<bool> {
        tokio::fs::metadata(path.as_path())
            .await
            .map_err(FileSystemError::read_error)
            .map(|_| true)
    }

    async fn read_file(&self, path: &CachePath) -> Result<Vec<u8>> {
        read_file(&make_cache_file_path(path.as_str())).await
    }

    async fn write_file(&self, path: &CachePath, content: &[u8]) -> Result<()> {
        write_file(&make_cache_file_path(path.as_str()), content).await
    }

    async fn download_and_cache_file(&self, url: &str) -> Result<CachePath> {
        self.get_file_from_cache_or_url(url).await?;

        let path = CachePath::from_url(url);

        Ok(path)
    }

    async fn get_file_from_cache_or_url(&self, url: &str) -> Result<Vec<u8>> {
        debug!("Getting file from cache or url: {}", url);

        let path = CachePath::from_url(url);

        if let Ok(content) = self.read_file(&path).await {
            return Ok(content);
        }

        let content = download_bytes(&self.reqwest_client, url).await?;

        self.write_file(&path, &content).await?;

        Ok(content)
    }
}
