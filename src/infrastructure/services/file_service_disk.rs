use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{spawn, task::JoinSet};
use tracing::debug;

use crate::{
    domain::services::FileService,
    error::{CsvError, FileSystemError, JsonError, YamlError},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct FileServiceDisk;

impl FileServiceDisk {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl FileService for FileServiceDisk {
    fn make_archive_file_path(&self, path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("ARCHIVE_DIR")).join(path)
    }

    fn make_content_file_path(&self, path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("CONTENT_DIR")).join(path)
    }

    fn make_cache_file_path(&self, path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        PathBuf::new().join(dotenv!("CACHE_DIR")).join(path)
    }

    fn make_output_file_path(&self, path: &Path) -> PathBuf {
        let path = path.strip_prefix("/").unwrap_or(path);

        Path::new(dotenv!("OUTPUT_DIR")).join(path)
    }

    fn make_file_path_from_date_and_file(
        &self,
        date: &DateTime<Utc>,
        file_name: &str,
        suffix: Option<&str>,
    ) -> PathBuf {
        let date_str = date.format("%Y/%m/%d").to_string();

        let file_name = file_name.split('/').last().unwrap();

        let name = file_name.split('.').next().unwrap();
        let ext = file_name.split('.').last().unwrap();

        let suffix_str = match suffix {
            Some(suffix) => format!("-{}", suffix),
            None => "".to_string(),
        };

        let path = format!("/{}/{}{}.{}", date_str, name, suffix_str, ext);

        Path::new(&path).to_path_buf()
    }

    async fn find_files_rescurse(&self, path: &Path, extension: &str) -> Result<Vec<String>> {
        debug!("Finding files in [{:?}]", path);

        let mut files = vec![];

        for entry in read_dir(path).map_err(FileSystemError::read_dir_error)? {
            let entry = entry.map_err(FileSystemError::read_dir_error)?;
            let path = entry.path();

            if path.is_dir() {
                let children = self.find_files_rescurse(&path, extension).await?;

                for child in children {
                    files.push(child);
                }
            } else if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path.to_str().unwrap().to_string());
                }
            }
        }

        Ok(files)
    }

    async fn get_file_metadata(&self, path: &Path) -> Result<std::fs::Metadata> {
        let metadata = tokio::fs::metadata(path)
            .await
            .map_err(FileSystemError::read_error)?;

        Ok(metadata)
    }

    async fn get_file_last_modified(&self, path: &Path) -> Result<DateTime<Utc>> {
        let metadata = self.get_file_metadata(path).await?;

        // TODO use git commit time if available

        let last_modified = metadata.modified().map_err(FileSystemError::read_error)?;

        Ok(DateTime::from(last_modified))
    }

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        debug!("Reading file from [{:?}]", path);
        let data = tokio::fs::read(path)
            .await
            .map_err(FileSystemError::read_error)?;

        Ok(data)
    }

    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()> {
        debug!("Writing file to [{:?}]", path);

        let parent_dir = path.parent().unwrap();

        tokio::fs::create_dir_all(parent_dir)
            .await
            .map_err(FileSystemError::create_dir_error)?;

        tokio::fs::write(path, data)
            .await
            .map_err(FileSystemError::write_error)
    }

    async fn read_json_file<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let data = self.read_file(path).await?;

        let data = serde_json::from_slice(&data).map_err(JsonError::parse_error)?;

        Ok(data)
    }

    async fn read_json_file_or_default<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        // Todo, behave different if file is not found vs unparsable
        match self.read_json_file(path).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(D::default()),
        }
    }

    async fn write_json_file<D>(&self, path: &Path, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        let data = serde_json::to_string(data).map_err(JsonError::stringify_error)?;

        self.write_file(path, data.as_bytes()).await
    }

    async fn read_csv_file<D>(&self, path: &Path) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        debug!("Reading csv from [{:?}]", path);
        let mut reader = csv::Reader::from_path(path).map_err(CsvError::read_error)?;
        let mut records = Vec::new();
        for record in reader.deserialize() {
            let record: D = record.map_err(CsvError::parse_error)?;
            records.push(record);
        }
        Ok(records)
    }

    async fn read_text_file(&self, path: &Path) -> Result<String> {
        debug!("Reading text from [{:?}]", path);
        tokio::fs::read_to_string(path)
            .await
            .map_err(FileSystemError::read_error)
    }

    async fn write_text_file_blocking(&self, path: &Path, data: &str) -> Result<()> {
        let data: Vec<u8> = data.as_bytes().to_vec();

        self.write_file(path, &data).await
    }

    async fn write_text_file(&self, path: PathBuf, data: String, join_set: &mut JoinSet<Result<()>>) -> Result<()> {
        let data: Vec<u8> = data.as_bytes().to_vec();
        join_set.spawn(async {
            let parent_dir = path.parent().unwrap();

            tokio::fs::create_dir_all(parent_dir)
                .await
                .map_err(FileSystemError::create_dir_error)?;

            tokio::fs::write(path, data)
                .await
                .map_err(FileSystemError::write_error)
        });

        Ok(())
    }

    async fn read_yaml_file<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let data = self.read_file(path).await?;

        let data = serde_yaml::from_slice(&data).map_err(YamlError::parse_error)?;

        Ok(data)
    }
}
