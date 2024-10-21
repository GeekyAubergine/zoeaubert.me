use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use tracing::debug;

use crate::{
    error::{CsvError, FileSystemError, JsonError},
    prelude::*,
};

pub fn make_archive_file_path(path: &Path) -> PathBuf {
    let path = path.strip_prefix("/").unwrap_or(path);

    PathBuf::new().join(dotenv!("ARCHIVE_DIR")).join(path)
}

pub fn make_content_file_path(path: &Path) -> PathBuf {
    let path = path.strip_prefix("/").unwrap_or(path);

    PathBuf::new().join(dotenv!("CONTENT_DIR")).join(path)
    // Path::new(dotenv!("CONTENT_DIR")).join(file_name)
}

pub fn make_cache_file_path(path: &Path) -> PathBuf {
    let path = path.strip_prefix("/").unwrap_or(path);

    PathBuf::new().join(dotenv!("CACHE_DIR")).join(path)
    // Path::new(dotenv!("CACHE_DIR")).join(file_name)
}

pub fn make_output_file_path(path: &Path) -> PathBuf {
    let path = path.strip_prefix("/").unwrap_or(path);

    Path::new(dotenv!("OUTPUT_DIR")).join(path)
}

pub fn make_file_path_from_date_and_file(
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

    let path = format!(
        "/{}/{}{}.{}",
        date_str,
        name,
        suffix_str,
        ext
    );

    Path::new(&path).to_path_buf()
}

pub fn find_files_rescurse(path: &Path, extension: &str) -> Result<Vec<String>> {
    debug!("Finding files in [{:?}]", path);

    let mut files = vec![];

    for entry in read_dir(path).map_err(FileSystemError::read_dir_error)? {
        let entry = entry.map_err(FileSystemError::read_dir_error)?;
        let path = entry.path();

        if path.is_dir() {
            let children = find_files_rescurse(&path, extension)?;

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

pub async fn get_file_metadata(path: &Path) -> Result<std::fs::Metadata> {
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(FileSystemError::read_error)?;

    Ok(metadata)
}

pub async fn get_file_last_modified(path: &Path) -> Result<DateTime<Utc>> {
    let metadata = get_file_metadata(path).await?;

    let last_modified = metadata.modified().map_err(FileSystemError::read_error)?;

    Ok(DateTime::from(last_modified))
}

pub async fn read_file(path: &Path) -> Result<Vec<u8>> {
    debug!("Reading file from [{:?}]", path);
    let data = tokio::fs::read(path)
        .await
        .map_err(FileSystemError::read_error)?;

    Ok(data)
}

pub async fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    debug!("Writing file to [{:?}]", path);

    let parent_dir = path.parent().unwrap();

    tokio::fs::create_dir_all(parent_dir)
        .await
        .map_err(FileSystemError::create_dir_error)?;

    tokio::fs::write(path, data)
        .await
        .map_err(FileSystemError::write_error)
}

pub async fn read_json_file<D>(path: &Path) -> Result<D>
where
    D: DeserializeOwned,
{
    let data = read_file(path).await?;

    let data = serde_json::from_slice(&data).map_err(JsonError::parse_error)?;

    Ok(data)
}

pub async fn read_json_file_or_default<D>(path: &Path) -> Result<D>
where
    D: DeserializeOwned + Default,
{
    match read_json_file(path).await {
        Ok(data) => Ok(data),
        Err(_) => Ok(D::default()),
    }
}

pub async fn write_json_file<D>(path: &Path, data: &D) -> Result<()>
where
    D: Serialize,
{
    let data = serde_json::to_string(data).map_err(JsonError::stringify_error)?;

    write_file(path, data.as_bytes()).await
}

pub async fn read_csv_file<D>(path: &Path) -> Result<Vec<D>>
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

pub async fn read_text_file(path: &Path) -> Result<String> {
    debug!("Reading text from [{:?}]", path);
    tokio::fs::read_to_string(path)
        .await
        .map_err(FileSystemError::read_error)
}

pub async fn write_text_file(path: &Path, data: &str) -> Result<()> {
    let data: Vec<u8> = data.as_bytes().to_vec();

    write_file(path, &data).await
}
