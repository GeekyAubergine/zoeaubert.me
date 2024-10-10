use std::{fs::read_dir, path::Path};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{de::DeserializeOwned, Serialize};
use tracing::debug;

use crate::{
    error::{CsvError, FileSystemError, JsonError},
    prelude::*,
};

pub fn make_archive_file_path(file_name: &str) -> String {
    format!("../{}/{}", dotenv!("ARCHIVE_DIR"), file_name)
}

pub fn make_content_file_path(file_name: &str) -> String {
    format!("../{}/{}", dotenv!("CONTENT_DIR"), file_name)
}

pub fn make_cache_file_path(file_name: &str) -> String {
    format!("../{}/{}", dotenv!("CACHE_DIR"), file_name)
}

pub async fn read_json_file<P, D>(path: &P) -> Result<D>
where
    P: AsRef<Path> + std::fmt::Display,
    D: DeserializeOwned,
{
    debug!("Reading json from [{}]", path);
    let data = tokio::fs::read_to_string(path)
        .await
        .map_err(FileSystemError::read_error)?;

    let data = serde_json::from_str(data.as_str()).map_err(JsonError::parse_error)?;

    Ok(data)
}

pub async fn read_json_file_or_default<P, D>(path: &P) -> Result<D>
where
    P: AsRef<Path> + std::fmt::Display,
    D: DeserializeOwned + Default,
{
    debug!("Reading json from [{}]", path);
    let data = match tokio::fs::read_to_string(path).await {
        Ok(d) => d,
        _ => return Ok(D::default()),
    };

    let data = serde_json::from_str(data.as_str()).map_err(JsonError::parse_error)?;

    Ok(data)
}

pub async fn write_json_file<P, D>(path: &P, data: &D) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
    D: Serialize,
{
    debug!("Writing json to [{}]", path);

    let parent_dir = path.as_ref().parent().unwrap();

    tokio::fs::create_dir_all(parent_dir)
        .await
        .map_err(FileSystemError::create_dir_error)?;

    let data = serde_json::to_string(data).map_err(JsonError::stringify_error)?;

    tokio::fs::write(path, data)
        .await
        .map_err(FileSystemError::write_error)
}

pub async fn get_file_last_modified<P>(path: &P) -> Result<DateTime<Utc>>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(FileSystemError::read_error)?;

    Ok(metadata
        .modified()
        .map_err(FileSystemError::read_error)?
        .into())
}

pub async fn read_csv_file<P, D>(path: &P) -> Result<Vec<D>>
where
    P: AsRef<Path> + std::fmt::Display,
    D: DeserializeOwned,
{
    debug!("Reading csv from [{}]", path);
    let mut reader = csv::Reader::from_path(path).map_err(CsvError::read_error)?;
    let mut records = Vec::new();
    for record in reader.deserialize() {
        let record: D = record.map_err(CsvError::parse_error)?;
        records.push(record);
    }
    Ok(records)
}

pub async fn read_text_file<P>(path: &P) -> Result<String>
where
    P: AsRef<Path> + std::fmt::Display,
{
    debug!("Reading text from [{}]", path);
    tokio::fs::read_to_string(path)
        .await
        .map_err(FileSystemError::read_error)
}

pub fn find_files_rescurse<P>(path: &P, extension: &str) -> Result<Vec<String>>
where
    P: AsRef<Path> + std::fmt::Display,
{
    debug!("Finding files in [{}]", path);

    let mut files = vec![];

    for entry in read_dir(path).map_err(FileSystemError::read_dir_error)? {
        let entry = entry.map_err(FileSystemError::read_dir_error)?;
        let path = entry.path();

        if path.is_dir() {
            let children = find_files_rescurse(&path.to_str().unwrap(), extension)?;

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
