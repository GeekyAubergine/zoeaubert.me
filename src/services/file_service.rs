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
pub struct CacheFile(PathBuf);

impl CacheFile {
    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

impl std::fmt::Display for CacheFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveFile(PathBuf);

impl ArchiveFile {
    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

impl std::fmt::Display for ArchiveFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentFile(PathBuf);

impl ContentFile {
    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

impl std::fmt::Display for ContentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputFile(PathBuf);

impl OutputFile {
    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

impl std::fmt::Display for OutputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetFile(PathBuf);

impl AssetFile {
    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

impl std::fmt::Display for AssetFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path().to_string_lossy())
    }
}

// -------- Read --------

fn read_file(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(FileSystemError::read_error)
}

fn read_text_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(FileSystemError::read_error)
}

fn read_json_file<D>(path: &Path) -> Result<D>
where
    D: DeserializeOwned,
{
    serde_json::from_slice(&read_file(path)?).map_err(JsonError::parse_error)
}

fn read_json_file_or_default<D>(path: &Path) -> Result<D>
where
    D: DeserializeOwned + Default,
{
    // Todo, behave different if file is not found vs unparsable
    match read_json_file(path) {
        Ok(data) => Ok(data),
        Err(_) => Ok(D::default()),
    }
}

fn read_yaml_file<D>(path: &Path) -> Result<D>
where
    D: DeserializeOwned,
{
    serde_yaml::from_slice(&read_file(path)?).map_err(YamlError::parse_error)
}

fn read_csv_file<D>(path: &Path) -> Result<Vec<D>>
where
    D: DeserializeOwned,
{
    let mut reader = csv::Reader::from_path(path).map_err(CsvError::read_error)?;
    let mut records = Vec::new();
    for record in reader.deserialize() {
        let record: D = record.map_err(CsvError::parse_error)?;
        records.push(record);
    }
    Ok(records)
}

fn file_exists(path: &Path) -> Result<bool> {
    if let Ok(_) = std::fs::metadata(path) {
        return Ok(true);
    }
    return Ok(false);
}

fn find_files_recursive(path: &Path, extension: &str) -> Result<Vec<String>> {
    debug!("Finding files in [{:?}]", path);

    let mut files = vec![];

    for entry in read_dir(path).map_err(FileSystemError::read_dir_error)? {
        let entry = entry.map_err(FileSystemError::read_dir_error)?;
        let path = entry.path();

        if path.is_dir() {
            let children = find_files_recursive(&path, extension)?;

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

pub trait ReadableFile {
    fn read(&self) -> Result<Vec<u8>>;

    fn read_text(&self) -> Result<String>;

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned;

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default;

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned;

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned;

    fn exists(&self) -> Result<bool>;

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>>;
}

impl ReadableFile for CacheFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&Path::new(dotenv!("CACHE_DIR")).join(&self.0))
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&Path::new(dotenv!("CACHE_DIR")).join(&self.0), extension)
    }
}

impl ReadableFile for ArchiveFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0))
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0), extension)
    }
}

impl ReadableFile for ContentFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0))
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&Path::new(dotenv!("CONTENT_DIR")).join(&self.0), extension)
    }
}

impl ReadableFile for AssetFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&Path::new("assets").join(&self.0))
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&Path::new("assets").join(&self.0))
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&Path::new("assets").join(&self.0))
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&Path::new("assets").join(&self.0))
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&Path::new("assets").join(&self.0))
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&Path::new("assets").join(&self.0))
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&Path::new("assets").join(&self.0))
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&Path::new("assets").join(&self.0), extension)
    }
}

// -------- Write --------

fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    debug!("Writing file to [{:?}]", path);

    let parent_dir = path.parent().unwrap();

    make_dir(&parent_dir)?;

    std::fs::write(path, data).map_err(FileSystemError::write_error)
}

fn write_text_file(path: &Path, data: &str) -> Result<()> {
    let data: Vec<u8> = data.as_bytes().to_vec();

    write_file(path, &data)
}

fn write_json_file<D>(path: &Path, data: &D) -> Result<()>
where
    D: Serialize + Send + Sync,
{
    let data = serde_json::to_string(data).map_err(JsonError::stringify_error)?;

    write_file(path, data.as_bytes())
}

pub trait WritableFile {
    fn write(&self, data: &[u8]) -> Result<()>;

    fn write_text(&self, data: &str) -> Result<()>;

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync;
}

impl WritableFile for CacheFile {
    fn write(&self, data: &[u8]) -> Result<()> {
        write_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&Path::new(dotenv!("CACHE_DIR")).join(&self.0), data)
    }
}

impl WritableFile for ArchiveFile {
    fn write(&self, data: &[u8]) -> Result<()> {
        write_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&Path::new(dotenv!("ARCHIVE_DIR")).join(&self.0), data)
    }
}

impl WritableFile for OutputFile {
    fn write(&self, data: &[u8]) -> Result<()> {
        write_file(&Path::new(dotenv!("OUTPUT_DIR")).join(&self.0), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&Path::new(dotenv!("OUTPUT_DIR")).join(&self.0), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&Path::new(dotenv!("OUTPUT_DIR")).join(&self.0), data)
    }
}

// -------- Utils

fn make_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path).map_err(FileSystemError::create_dir_error)
}

pub struct FileService;

impl FileService {
    pub fn cache(path: PathBuf) -> CacheFile {
        CacheFile(path)
    }

    pub fn archive(path: PathBuf) -> ArchiveFile {
        ArchiveFile(path)
    }

    pub fn content(path: PathBuf) -> ContentFile {
        ContentFile(path)
    }

    pub fn output(path: PathBuf) -> OutputFile {
        OutputFile(path)
    }

    pub fn asset(path: PathBuf) -> AssetFile {
        AssetFile(path)
    }

    pub fn copy(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying [{:?}] to [{:?}]", source, destination);

        std::fs::copy(source, destination).map_err(FileSystemError::copy_file_error)?;

        Ok(())
    }

    pub fn copy_dir(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying directory [{:?}] to [{:?}]", source, destination);
        copy_dir(source, destination).map_err(FileSystemError::copy_dir_error)?;

        Ok(())
    }
}
