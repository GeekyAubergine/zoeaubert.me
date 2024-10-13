use super::models::{cache_path::CachePath, media::Media};

use crate::prelude::*;

#[async_trait::async_trait]
pub trait CacheService {
    async fn is_file_cached(&self, path: &CachePath) -> Result<bool>;

    async fn read_file(&self, path: &CachePath) -> Result<Vec<u8>>;

    async fn write_file(&self, path: &CachePath, content: &[u8]) -> Result<()>;

    async fn get_file_from_cache_or_url(&self, url: &str) -> Result<Vec<u8>>;
}

#[async_trait::async_trait]
pub trait CdnService {
    async fn copy_file(&self, source: &str, destination: &str) -> Result<()>;
}
