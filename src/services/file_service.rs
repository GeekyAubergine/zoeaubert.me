use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dircpy::copy_dir;
use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use tokio::task::JoinSet;
use tracing::debug;

use crate::{
    error::{CsvError, FileSystemError, JsonError, YamlError},
    prelude::*,
};

pub struct FileService;

impl FileService {
    pub fn make_archive_file_path(path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("ARCHIVE_DIR")).join(path)
    }

    pub fn make_content_file_path(path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("CONTENT_DIR")).join(path)
    }

    pub fn make_cache_file_path(path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("CACHE_DIR")).join(path)
    }

    pub fn make_output_file_path(path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        Path::new(dotenv!("OUTPUT_DIR")).join(path)
    }

    pub async fn write_text_file(path: &Path, data: &str) -> Result<()> {
        let data: Vec<u8> = data.as_bytes().to_vec();

        Self::write_file(path, &data).await
    }

    // -------- Utils

    async fn write_file(path: &Path, data: &[u8]) -> Result<()> {
        debug!("Writing file to [{:?}]", path);

        let parent_dir = path.parent().unwrap();

        Self::make_dir(parent_dir).await?;

        tokio::fs::write(path, data)
            .await
            .map_err(FileSystemError::write_error)
    }

    async fn make_dir( path: &Path) -> Result<()> {
        debug!("Making directory [{:?}]", path);
        tokio::fs::create_dir_all(path)
            .await
            .map_err(FileSystemError::create_dir_error)
    }
}
