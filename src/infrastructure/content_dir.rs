use std::{fs, path::Path};

use tracing::{debug, info};

use crate::{error::Error, prelude::*};

use super::config::Config;

#[derive(Debug, Clone)]
pub struct ContentDir {}

impl ContentDir {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn read_file(&self, path: &str, config: &Config) -> Result<String> {
        let path = match path.starts_with(config.content_dir()) {
            true => path.to_string(),
            false => format!("{}/{}", config.content_dir(), path),
        };
        let path = Path::new(&path);

        debug!("Reading file: {:?}", path);

        tokio::fs::read_to_string(path)
            .await
            .map_err(Error::FileSystemUnreadable)
    }
}
