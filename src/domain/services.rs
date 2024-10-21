use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use url::Url;

use super::{
    models::{cache_path::CachePath, image::Image, media::Media},
    state::State,
};

use crate::prelude::*;

#[async_trait::async_trait]
pub trait CacheService {
    async fn is_file_cached(&self, path: &Path) -> Result<bool>;

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>>;

    async fn write_file(&self, path: &Path, content: &[u8]) -> Result<()>;

    async fn download_and_cache_file(&self, url: &Url) -> Result<()>;

    async fn get_file_from_cache_or_url(&self, url: &Url) -> Result<(PathBuf, Vec<u8>)>;
}

#[async_trait::async_trait]
pub trait CdnService {
    async fn upload_file(&self, source: &Path, destination: &Path) -> Result<()>;
}
