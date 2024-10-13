use dotenvy_macro::dotenv;
use std::{fmt::Display, path::Path};

const CACHE_DIR: &str = dotenv!("CACHE_DIR");

#[derive(Debug, thiserror::Error)]
pub enum CacheDirError {
    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
    #[error("Url download {0}")]
    UrlDownload(reqwest::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CachePath(String);

impl CachePath {
    pub fn from_str(path: &str) -> Result<Self, CacheDirError> {
        if path.starts_with("/") {
            Ok(Self(path.to_string()))
        } else {
            Ok(Self(format!("{}/{}", CACHE_DIR, path)))
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, CacheDirError> {
        Self::from_str(path.to_str().unwrap())
    }

    pub fn from_url(url: &str) -> Result<Self, CacheDirError> {
        let path = url.split('/').skip(3).collect::<Vec<&str>>().join("/");
        Self::from_str(&path)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        &Path::new(&self.0)
    }
}

impl Display for CachePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait::async_trait]
pub trait CacheService {
    async fn is_file_cached(&self, path: &CachePath) -> Result<bool, CacheDirError>;

    async fn read_file(&self, path: &CachePath) -> Result<Vec<u8>, CacheDirError>;

    async fn write_file(&self, path: &CachePath, content: Vec<u8>) -> Result<(), CacheDirError>;

    async fn get_file_from_cache_or_url(&self, url: &str) -> Result<Vec<u8>, CacheDirError>;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultCache {
    reqwest_client: reqwest::Client,
}

impl DefaultCache {
    pub fn new() -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl CacheService for DefaultCache {
    async fn is_file_cached(&self, path: &CachePath) -> Result<bool, CacheDirError> {
        tokio::fs::metadata(path.as_path())
            .await
            .map_err(CacheDirError::FileSystemError)
            .map(|_| true)
    }

    async fn read_file(&self, path: &CachePath) -> Result<Vec<u8>, CacheDirError> {
        tokio::fs::read(path.as_path())
            .await
            .map_err(CacheDirError::FileSystemError)
    }

    async fn write_file(&self, path: &CachePath, content: Vec<u8>) -> Result<(), CacheDirError> {
        tokio::fs::write(path.as_path(), content)
            .await
            .map_err(CacheDirError::FileSystemError)
    }

    async fn get_file_from_cache_or_url(&self, url: &str) -> Result<Vec<u8>, CacheDirError> {
        let path = CachePath::from_url(url)?;

        if let Ok(content) = self.read_file(&path).await {
            return Ok(content);
        }

        let request = self
            .reqwest_client
            .get(url)
            .send()
            .await
            .map_err(CacheDirError::UrlDownload)?;

        let content = request
            .bytes()
            .await
            .map_err(CacheDirError::UrlDownload)?
            .to_vec();

        self.write_file(&path, content.to_vec()).await?;

        Ok(content.to_vec())
    }
}
