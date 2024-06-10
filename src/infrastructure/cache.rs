use std::path::Path;

use crate::{error::Error, prelude::*};

use super::config::Config;

#[derive(Debug, Clone, Default)]
pub struct Cache {}

impl Cache {
    pub async fn is_file_cached(&self, path: &str, config: &Config) -> Result<bool> {
        let path = format!("{}/{}", config.cache_dir(), path);
        let path = Path::new(&path);

        tokio::fs::try_exists(path)
            .await
            .map_err(Error::FileSystemUnreadable)
    }

    pub async fn read_cached_file(&self, path: &str, config: &Config) -> Result<Option<String>> {
        let path = format!("{}/{}", config.cache_dir(), path);
        let path = Path::new(&path);

        match tokio::fs::read_to_string(path).await {
            Ok(content) => Ok(Some(content)),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(None)
                } else {
                    Err(Error::FileSystemUnreadable(e))
                }
            }
        }
    }
}
