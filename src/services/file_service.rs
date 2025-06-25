use std::{
    borrow::Cow,
    f64::consts::PI,
    ffi::OsStr,
    fs::read_dir,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dircpy::copy_dir;
use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::task::JoinSet;
use tracing::debug;
use url::Url;

use crate::{
    error::{CsvError, FileSystemError, JsonError, YamlError},
    prelude::*,
    services::ServiceContext,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FilePath {
    path: PathBuf,
}

impl FilePath {
    pub fn archive(path: &str) -> Self {
        let path = path.strip_prefix("/").unwrap_or(path);

        Self {
            path: PathBuf::new().join(dotenv!("ARCHIVE_DIR")).join(path),
        }
    }

    pub fn assets(path: &str) -> Self {
        let path = path.strip_prefix("/").unwrap_or(path);

        Self {
            path: PathBuf::new().join("assets").join(path),
        }
    }


    pub fn content(path: &str) -> Self {
        let path = path.strip_prefix("/").unwrap_or(path);

        Self {
            path: PathBuf::new().join(dotenv!("CONTENT_DIR")).join(path),
        }
    }

    pub fn cache(path: &str) -> Self {
        let path = path.strip_prefix("/").unwrap_or(path);

        Self {
            path: PathBuf::new().join(dotenv!("CACHE_DIR")).join(path),
        }
    }

    pub fn output(path: &str) -> Self {
        let path = path.strip_prefix("/").unwrap_or(path);

        Self {
            path: PathBuf::new().join(dotenv!("OUTPUT_DIR")).join(path),
        }
    }

    pub fn path_from_date_and_file_name(
        date: &DateTime<Utc>,
        file_name: &str,
        suffix: Option<&str>,
    ) -> Self {
        let date_str = date.format("%Y/%m/%d").to_string();

        let file_name = file_name.split('/').last().unwrap();

        let name = file_name.split('.').next().unwrap();
        let ext = file_name.split('.').last().unwrap();

        let suffix_str = match suffix {
            Some(suffix) => format!("-{}", suffix),
            None => "".to_string(),
        };

        let path = format!("/{}/{}{}.{}", date_str, name, suffix_str, ext);

        Self {
            path: Path::new(&path).to_path_buf()
        }
    }

    // ---- Read

    pub async fn read(&self) -> Result<Vec<u8>> {
        FileService::read_file(&self.path).await
    }

    pub async fn read_text(&self) -> Result<String> {
        FileService::read_text_file(&self.path).await
    }

    pub async fn read_as_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        FileService::read_json_file(&self.path).await
    }

    pub async fn read_as_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        FileService::read_json_file_or_default(&self.path).await
    }

    pub async fn read_as_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        FileService::read_yaml_file(&self.path).await
    }

    pub async fn read_as_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        FileService::read_csv_file(&self.path).await
    }

    // -------- Write

    pub async fn write(&self, data: &[u8]) -> Result<()> {
        FileService::write_file(&self.path, data).await
    }

    pub async fn write_text(&self, data: &str) -> Result<()> {
        FileService::write_text_file(&self.path, data).await
    }

    pub async fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        FileService::write_json_file(&self.path, data).await
    }

    // -------- Utils

    pub async fn make_dir(&self) -> Result<()> {
        FileService::make_dir(&self.path).await
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }

    pub fn parent(&self) -> Option<FilePath> {
        self.path.parent().map(|parent| FilePath {
            path: PathBuf::from(parent),
        })
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }

    pub fn file_name(&self) -> Option<&OsStr> {
        self.path.file_name()
    }

    pub async fn exists(&self) -> Result<bool> {
        tokio::fs::metadata(&self.path)
            .await
            .map_err(FileSystemError::read_error)
            .map(|_| true)
    }

    pub fn starts_with(&self, search: &str) -> bool {
        self.path.starts_with(search)
    }

    pub fn as_url(&self) -> Option<Result<Url>> {
        self.path
            .to_str()
            .map(|path| path.parse().map_err(FileSystemError::path_is_not_url))
    }

    pub fn find_recurisve_files(&self, extension: &str) -> Result<Vec<FilePath>> {
        let paths = FileService::find_files_recursive(&self.path, extension)?;

        Ok(paths
            .iter()
            .map(|path| FilePath { path: path.into() })
            .collect())
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}


pub struct FileService;

impl FileService {
    // -------- Read
    async fn read_file(path: &Path) -> Result<Vec<u8>> {
        debug!("Reading file from [{:?}]", path);
        let data = tokio::fs::read(path)
            .await
            .map_err(FileSystemError::read_error)?;

        Ok(data)
    }

    async fn read_text_file(path: &Path) -> Result<String> {
        debug!("Reading text from [{:?}]", path);
        tokio::fs::read_to_string(path)
            .await
            .map_err(FileSystemError::read_error)
    }

    async fn read_json_file<D>(path: &Path) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let data = Self::read_file(path).await?;

        let data = serde_json::from_slice(&data).map_err(JsonError::parse_error)?;

        Ok(data)
    }

    async fn read_json_file_or_default<D>(path: &Path) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        // Todo, behave different if file is not found vs unparsable
        match Self::read_json_file(path).await {
            Ok(data) => Ok(data),
            Err(_) => Ok(D::default()),
        }
    }

    async fn read_yaml_file<D>(path: &Path) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let data = Self::read_file(path).await?;

        let data = serde_yaml::from_slice(&data).map_err(YamlError::parse_error)?;

        Ok(data)
    }

    async fn read_csv_file<D>(path: &Path) -> Result<Vec<D>>
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

    // -------- Write

    async fn write_file(path: &Path, data: &[u8]) -> Result<()> {
        debug!("Writing file to [{:?}]", path);

        let parent_dir = path.parent().unwrap();

        Self::make_dir(parent_dir).await?;

        tokio::fs::write(path, data)
            .await
            .map_err(FileSystemError::write_error)
    }

    async fn write_text_file(path: &Path, data: &str) -> Result<()> {
        let data: Vec<u8> = data.as_bytes().to_vec();

        Self::write_file(path, &data).await
    }

    async fn write_json_file<D>(path: &Path, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        let data = serde_json::to_string(data).map_err(JsonError::stringify_error)?;

        Self::write_file(path, data.as_bytes()).await
    }

    // -------- Utils

    pub async fn make_dir(path: &Path) -> Result<()> {
        debug!("Making directory [{:?}]", path);
        tokio::fs::create_dir_all(path)
            .await
            .map_err(FileSystemError::create_dir_error)
    }

    fn find_files_recursive(path: &Path, extension: &str) -> Result<Vec<String>> {
        debug!("Finding files in [{:?}]", path);

        let mut files = vec![];

        for entry in read_dir(path).map_err(FileSystemError::read_dir_error)? {
            let entry = entry.map_err(FileSystemError::read_dir_error)?;
            let path = entry.path();

            if path.is_dir() {
                let children = Self::find_files_recursive(&path, extension)?;

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

    pub async fn copy(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying [{:?}] to [{:?}]", source, destination);
        tokio::fs::copy(source, destination)
            .await
            .map_err(FileSystemError::copy_file_error)?;

        Ok(())
    }

    pub async fn copy_dir(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying directory [{:?}] to [{:?}]", source, destination);
        copy_dir(source, destination).map_err(FileSystemError::copy_dir_error)?;

        Ok(())
    }
}
