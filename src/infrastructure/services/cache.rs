use std::{fmt::Display, path::Path};

use imagesize::{blob_size, ImageSize};
use reqwest::ClientBuilder;

use crate::{
    domain::models::media::image::Image,
    error::Error,
    infrastructure::{app_state::AppState, config::Config},
    prelude::*,
};

use super::cdn::{Cdn, CdnPath};

#[derive(Debug, Clone)]
pub struct CachePath(String);

impl CachePath {
    pub fn new(config: &Config, path: String) -> Self {
        Self(format!("{}/{}", config.cache_dir(), path))
    }

    pub fn from_url(config: &Config, url: &str) -> Self {
        let path = url.split('/').skip(3).collect::<Vec<&str>>().join("/");
        Self::new(config, path)
    }

    pub fn path(&self) -> &str {
        &self.0
    }

    pub fn cdn_path(&self, config: &Config) -> CdnPath {
        CdnPath::new(self.0.replace(config.cache_dir(), ""))
    }
}

impl Display for CachePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cache {
    reqwest_client: reqwest::Client,
}

impl Cache {
    pub async fn new(config: &Config) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        Self {
            reqwest_client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

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
        cache_path: &CachePath,
        url: &str,
    ) -> Result<Vec<u8>> {
        if let Ok(content) = self.read_cached_file(cache_path, app_state.config()).await {
            return Ok(content);
        }

        let cdn = app_state.cdn();
        let config = app_state.config();

        let reqwest = self
            .reqwest_client
            .get(url)
            .send()
            .await
            .map_err(Error::UrlDownload)?;

        let content = reqwest.bytes().await.map_err(Error::UrlDownload)?.to_vec();

        self.cache_file(cache_path, content.clone()).await?;

        Ok(content)
    }

    pub async fn get_file_from_cache_or_download_from_cdn(
        &self,
        app_state: &AppState,
        cache_path: &CachePath,
    ) -> Result<Vec<u8>> {
        let config = app_state.config();
        let cdn = app_state.cdn();

        let cdn_path = cache_path.cdn_path(config);

        if let Ok(content) = self.read_cached_file(cache_path, config).await {
            return Ok(content);
        }

        let content = cdn.download_file(&cdn_path, config).await?;

        self.cache_file(cache_path, content.clone()).await?;

        Ok(content)
    }

    pub async fn get_image_size_from_cache_or_download(
        &self,
        app_state: &AppState,
        path: &CachePath,
    ) -> Result<ImageSize> {
        let content = self
            .get_file_from_cache_or_download_from_cdn(app_state, path)
            .await?;

        match blob_size(&content) {
            Ok(size) => Ok(size),
            Err(e) => Err(Error::ImageSize(e.to_string())),
        }
    }
}
