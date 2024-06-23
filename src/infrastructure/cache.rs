use std::path::Path;

use imagesize::{blob_size, ImageSize};

use crate::{domain::models::media::image::Image, error::Error, prelude::*};

use super::{
    app_state::AppState,
    cdn::{Cdn, CdnPath},
    config::Config,
};

#[derive(Debug, Clone)]
pub struct CachePath(String);

impl CachePath {
    pub fn new(config: &Config, path: String) -> Self {
        Self(format!("{}/{}", config.cache_dir(), path))
    }

    pub fn path(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cache {}

impl Cache {
    pub async fn is_file_cached(&self, path: &CachePath, config: &Config) -> Result<bool> {
        tokio::fs::try_exists(Path::new(&path.path()))
            .await
            .map_err(Error::FileSystemUnreadable)
    }

    pub async fn read_cached_file(&self, path: &CachePath, config: &Config) -> Result<Vec<u8>> {
        match tokio::fs::read(Path::new(&path.path())).await {
            Ok(content) => Ok(content),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Err(Error::FileNotFound(path.path().to_string()));
                }
                Err(Error::FileSystemUnreadable(e))
            }
        }
    }

    pub async fn cache_file(&self, path: &CachePath, content: Vec<u8>) -> Result<()> {
        let path = Path::new(path.path());

        // Make folders
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .map_err(Error::FileSystemUnwritable)?;

        tokio::fs::write(path, content)
            .await
            .map_err(Error::FileSystemUnwritable)
    }

    pub async fn get_file_from_cache_or_download(
        &self,
        app_state: &AppState,
        path: &str,
    ) -> Result<Vec<u8>> {
        let config = app_state.config();
        let cdn = app_state.cdn();

        let cache_path = CachePath::new(config, path.to_string());
        let cdn_path = CdnPath::new(path.to_string());

        if let Ok(content) = self.read_cached_file(&cache_path, config).await {
            return Ok(content);
        }

        let content = cdn.download_file(&cdn_path, config).await?;

        self.cache_file(&cache_path, content.clone()).await?;

        Ok(content)
    }

    pub async fn get_image_size_from_cache_or_download(
        &self,
        app_state: &AppState,
        path: &str,
    ) -> Result<ImageSize> {
        let content = self
            .get_file_from_cache_or_download(app_state, path)
            .await?;

        match blob_size(&content) {
            Ok(size) => Ok(size),
            Err(e) => Err(Error::ImageSize(e.to_string())),
        }
    }
}
