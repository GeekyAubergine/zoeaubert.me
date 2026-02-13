use std::{
    fs::read_dir, path::{Path, PathBuf}
};

use dircpy::copy_dir;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::debug;

use crate::{
    error::{CsvError, FileSystemError, JsonError, YamlError},
    prelude::*,
};

const CACHE_DIR: &str = ".cache";
const CONTENT_DIR: &str = "content";
const ARCHIVE_DIR: &str = ".archive";
const OUTPUT_DIR: &str = "output";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheFile(PathBuf);

impl CacheFile {
    pub fn as_path_buff(&self) -> PathBuf {
        Path::new(CACHE_DIR).join(&self.0)
    }
}

impl std::fmt::Display for CacheFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_buff().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveFile(PathBuf);

impl ArchiveFile {
    pub fn as_path_buff(&self) -> PathBuf {
        Path::new(ARCHIVE_DIR).join(&self.0)
    }
}

impl std::fmt::Display for ArchiveFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_buff().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentFile(PathBuf);

impl ContentFile {
    pub fn as_path_buff(&self) -> PathBuf {
        Path::new(CONTENT_DIR).join(&self.0)
    }
}

impl std::fmt::Display for ContentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_buff().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputFile(PathBuf);

impl OutputFile {
    pub fn as_path_buff(&self) -> PathBuf {
        Path::new(OUTPUT_DIR).join(&self.0)
    }
}

impl std::fmt::Display for OutputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_buff().to_string_lossy())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetFile(PathBuf);

impl AssetFile {
    pub fn as_path_buff(&self) -> PathBuf {
        Path::new("assets").join(&self.0)
    }
}

impl std::fmt::Display for AssetFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_buff().to_string_lossy())
    }
}

// -------- Read --------

fn read_file(path: &Path) -> Result<Vec<u8>> {
    debug!("Reading file [{:?}]", path);
    std::fs::read(path).map_err(FileSystemError::read_error)
}

fn read_text_file(path: &Path) -> Result<String> {
    debug!("Reading file [{:?}]", path);
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
    debug!("Reading file [{:?}]", path);
    let mut reader = csv::Reader::from_path(path).map_err(CsvError::read_error)?;
    let mut records = Vec::new();
    for record in reader.deserialize() {
        let record: D = record.map_err(CsvError::parse_error)?;
        records.push(record);
    }
    Ok(records)
}

fn file_exists(path: &Path) -> Result<bool> {
    if std::fs::metadata(path).is_ok() {
        return Ok(true);
    }
    Ok(false)
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
        } else if let Some(ext) = path.extension()
            && ext == extension {
                files.push(path.to_str().unwrap().to_string());
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
        read_file(&self.as_path_buff())
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&self.as_path_buff())
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&self.as_path_buff())
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&self.as_path_buff())
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&self.as_path_buff())
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&self.as_path_buff())
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&self.as_path_buff())
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&self.as_path_buff(), extension)
    }
}

impl ReadableFile for ArchiveFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&self.as_path_buff())
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&self.as_path_buff())
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&self.as_path_buff())
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&self.as_path_buff())
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&self.as_path_buff())
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&self.as_path_buff())
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&self.as_path_buff())
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&self.as_path_buff(), extension)
    }
}

impl ReadableFile for ContentFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&self.as_path_buff())
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&self.as_path_buff())
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&self.as_path_buff())
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&self.as_path_buff())
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&self.as_path_buff())
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&self.as_path_buff())
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&self.as_path_buff())
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&self.as_path_buff(), extension)
    }
}

impl ReadableFile for AssetFile {
    fn read(&self) -> Result<Vec<u8>> {
        read_file(&self.as_path_buff())
    }

    fn read_text(&self) -> Result<String> {
        read_text_file(&self.as_path_buff())
    }

    fn read_json<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_json_file(&self.as_path_buff())
    }

    fn read_json_or_default<D>(&self) -> Result<D>
    where
        D: DeserializeOwned + Default,
    {
        read_json_file_or_default(&self.as_path_buff())
    }

    fn read_yaml<D>(&self) -> Result<D>
    where
        D: DeserializeOwned,
    {
        read_yaml_file(&self.as_path_buff())
    }

    fn read_csv<D>(&self) -> Result<Vec<D>>
    where
        D: DeserializeOwned,
    {
        read_csv_file(&self.as_path_buff())
    }

    fn exists(&self) -> Result<bool> {
        file_exists(&self.as_path_buff())
    }

    fn find_files_recursive(&self, extension: &str) -> Result<Vec<String>> {
        find_files_recursive(&self.as_path_buff(), extension)
    }
}

// -------- Write --------

fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    debug!("Writing file to [{:?}]", path);

    let parent_dir = path.parent().unwrap();

    make_dir(parent_dir)?;

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
        write_file(&self.as_path_buff(), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&self.as_path_buff(), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&self.as_path_buff(), data)
    }
}

impl WritableFile for ArchiveFile {
    fn write(&self, data: &[u8]) -> Result<()> {
        write_file(&self.as_path_buff(), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&self.as_path_buff(), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&self.as_path_buff(), data)
    }
}

impl WritableFile for OutputFile {
    fn write(&self, data: &[u8]) -> Result<()> {
        write_file(&self.as_path_buff(), data)
    }

    fn write_text(&self, data: &str) -> Result<()> {
        write_text_file(&self.as_path_buff(), data)
    }

    fn write_json<D>(&self, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync,
    {
        write_json_file(&self.as_path_buff(), data)
    }
}

// -------- Utils

fn make_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path).map_err(FileSystemError::create_dir_error)
}

fn clean_path(path: PathBuf) -> PathBuf {
    match path.starts_with("/") {
        true => path.strip_prefix("/").unwrap().to_path_buf(),
        false => path,
    }
}

pub struct FileService;

impl FileService {
    pub fn cache(path: PathBuf) -> CacheFile {
        let path = match path.starts_with(CACHE_DIR) {
            true => path.strip_prefix(CACHE_DIR).unwrap().to_path_buf(),
            false => path,
        };

        CacheFile(clean_path(path))
    }

    pub fn archive(path: PathBuf) -> ArchiveFile {
        let path = match path.starts_with(ARCHIVE_DIR) {
            true => path.strip_prefix(ARCHIVE_DIR).unwrap().to_path_buf(),
            false => path,
        };

        ArchiveFile(clean_path(path))
    }

    pub fn content(path: PathBuf) -> ContentFile {
        let path = match path.starts_with(CONTENT_DIR) {
            true => path.strip_prefix(CONTENT_DIR).unwrap().to_path_buf(),
            false => path,
        };

        ContentFile(clean_path(path))
    }

    pub fn output(path: PathBuf) -> OutputFile {
        let path = match path.starts_with(OUTPUT_DIR) {
            true => path.strip_prefix(OUTPUT_DIR).unwrap().to_path_buf(),
            false => path,
        };

        OutputFile(clean_path(path))
    }

    pub fn asset(path: PathBuf) -> AssetFile {
        let path = match path.starts_with("assets") {
            true => path.strip_prefix("assets").unwrap().to_path_buf(),
            false => path,
        };

        AssetFile(clean_path(path))
    }

    pub fn copy(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying [{:?}] to [{:?}]", source, destination);

        make_dir(destination.parent().unwrap())?;

        std::fs::copy(source, destination).map_err(FileSystemError::copy_file_error)?;

        Ok(())
    }

    pub fn copy_dir(source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying directory [{:?}] to [{:?}]", source, destination);

        copy_dir(source, destination).map_err(FileSystemError::copy_dir_error)?;

        Ok(())
    }
}
