use std::path::{Path, PathBuf};

use image::ImageError;
use imagesize::{blob_size, ImageSize};
use tracing::{debug, info};
use url::Url;

use crate::{
    domain::{
        models::cache_path::CachePath,
        services::{CacheService, FileService, NetworkService},
        state::State,
    },
    error::{FileSystemError, NetworkError},
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

    async fn read_file(&self, state: &impl State, path: &Path) -> Result<Vec<u8>> {
        state
            .file_service()
            .read_file(&state.file_service().make_cache_file_path(path))
            .await
    }

    async fn write_file(&self, state: &impl State, path: &Path, content: &[u8]) -> Result<()> {
        state
            .file_service()
            .write_file(&state.file_service().make_cache_file_path(path), content)
            .await
    }
}
