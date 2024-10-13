use tracing::{debug, info};

use crate::{
    domain::{models::cache_path::CachePath, services::CacheService},
    error::{FileSystemError, NetworkError},
    infrastructure::utils::file_system::{make_cache_file_path, read_file, write_file},
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

    async fn get_file_from_cache_or_url(&self, url: &str) -> Result<Vec<u8>> {
        let path = CachePath::from_url(url);

        if let Ok(content) = self.read_file(&path).await {
            return Ok(content);
        }

        info!("Fetching file from URL: {}", url);

        let request = self
            .reqwest_client
            .get(url)
            .send()
            .await
            .map_err(NetworkError::fetch_error)?;

        let content = request
            .bytes()
            .await
            .map_err(NetworkError::fetch_error)?
            .to_vec();

        self.write_file(&path, &content).await?;

        Ok(content)
    }
}
