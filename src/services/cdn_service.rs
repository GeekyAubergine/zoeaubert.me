use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::exit,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use reqwest::{blocking::Body, blocking::ClientBuilder, header::ACCEPT};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::{debug, info};
use url::Url;

use crate::{
    config::{CONFIG, Config},
    error::{CdnError, Error, FileSystemError, NetworkError},
    prelude::*,
    services::{
        ServiceContext,
        file_service::{CacheFile, FileService, ReadableFile},
    },
};

fn make_cdn_api_url(path: &str) -> Url {
    format!("{}{}", CONFIG.bunny_cdn.url, path).parse().unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct CdnFile {
    directory: String,
    file_name: String,
    extension: String,
}

impl CdnFile {
    pub fn from_str(string: &str) -> Self {
        let path = PathBuf::from(string);

        let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
        let extension = path.extension().unwrap().to_string_lossy().to_string();
        let directory = path.parent().unwrap().to_string_lossy().to_string();

        Self {
            file_name,
            extension,
            directory,
        }
    }

    pub fn from_date_and_file_name(
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

        let path = format!("{}/{}{}.{}", date_str, name, suffix_str, ext);

        Self::from_str(&path)
    }

    pub fn add_suffix_to_file_name(&self, suffix: &str) -> Self {
        Self {
            directory: self.directory.clone(),
            file_name: format!("{}{}", self.file_name, suffix),
            extension: self.extension.clone(),
        }
    }

    pub fn as_string(&self) -> String {
        format!("{}/{}.{}", self.directory, self.file_name, self.extension)
    }

    pub fn as_cache_file(&self) -> CacheFile {
        debug!(
            "as cache file [{:?}] [{:?}]",
            FileService::cache(PathBuf::from(&self.as_string())),
            &self
        );

        FileService::cache(PathBuf::from(&self.as_string()))
    }

    fn as_cdn_api_url(&self) -> Url {
        make_cdn_api_url(&self.as_string())
    }

    pub fn as_cdn_url(&self) -> Url {
        let path = format!("{}/{}", CONFIG.cdn_url, self.as_string());

        path.parse().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BunnyCdnFileResponse {
    #[serde(rename = "StorageZoneName")]
    storage_zone_name: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "ObjectName")]
    object_name: String,
}

impl BunnyCdnFileResponse {
    pub fn path(&self) -> String {
        let path = format!("{}{}", self.path, self.object_name);
        let removable_prefix = format!("/{}/", self.storage_zone_name);
        path.replace(&removable_prefix, "")
    }
}

impl From<BunnyCdnFileResponse> for CdnFile {
    fn from(value: BunnyCdnFileResponse) -> Self {
        Self::from_str(&value.path())
    }
}

impl From<&BunnyCdnFileResponse> for CdnFile {
    fn from(value: &BunnyCdnFileResponse) -> Self {
        Self::from_str(&value.path())
    }
}

#[derive(Debug)]
pub struct CdnService {
    reqwest_client: reqwest::blocking::Client,
    existing_folders_cache: Arc<RwLock<HashSet<CdnFile>>>,
}

impl CdnService {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "AccessKey",
            CONFIG.bunny_cdn.access_key.parse().unwrap(),
        );
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        Self {
            reqwest_client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
            existing_folders_cache: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    fn query_file_directory(&self, file: &CdnFile) -> Result<Option<Vec<BunnyCdnFileResponse>>> {
        debug!("CdnService | Querying path in cdn: {:?}", &file.directory);

        let response = self
            .reqwest_client
            .get(make_cdn_api_url(&file.directory).as_str())
            .send()
            .map_err(NetworkError::fetch_error)?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        match response.json::<Vec<BunnyCdnFileResponse>>() {
            Ok(response) => Ok(Some(response)),
            Err(e) => Err(NetworkError::fetch_error(e)),
        }
    }

    fn files_names_in_folder(&self, path: &CdnFile) -> Result<Vec<String>> {
        debug!(
            "CdnService | Querying files in folder: [{}]",
            path.as_string(),
        );

        match self.query_file_directory(&path) {
            Ok(Some(files)) => {
                let mut cache = self.existing_folders_cache.write().unwrap();

                for file in files.iter() {
                    cache.insert(file.into());
                }

                dbg!(cache);

                Ok(files.iter().map(|f| f.object_name.clone()).collect())
            }
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    fn file_exists(&self, file: &CdnFile) -> Result<bool> {
        debug!("CdnService | Does file exist");

        if let Some(file) = self.existing_folders_cache.read().unwrap().get(&file) {
            debug!("CdnService | File exists in cache: {:?}", file);
            return Ok(true);
        }

        debug!(
            "CdnService | Querying files in folder: [{}]",
            file.as_string(),
        );

        match self.query_file_directory(&file) {
            Ok(Some(files)) => {
                let mut cache = self.existing_folders_cache.write().unwrap();

                for file in files.iter() {
                    cache.insert(file.into());
                }

                dbg!(cache);
            }
            Ok(None) => {}
            Err(e) => {
                // TODO log
            }
        };

        Ok(self
            .existing_folders_cache
            .read()
            .unwrap()
            .get(&file)
            .is_some())
    }

    pub fn upload_file(
        &self,
        ctx: &ServiceContext,
        file: &CacheFile,
        cdn_file: &CdnFile,
    ) -> Result<()> {
        if self.file_exists(cdn_file)? {
            return Ok(());
        }

        debug!(
            "CdnService | Uploading [{}]",
            file.as_path_buff().to_string_lossy(),
        );

        let data = file.read()?;

        let request = self
            .reqwest_client
            .put(cdn_file.as_cdn_api_url())
            .header("Content-Type", "application/octet-stream")
            .body(data);

        let response = request.send().map_err(CdnError::upload_error)?;

        if response.status().as_u16() != 201 {
            return Err(CdnError::base_status(response.status().as_u16()));
        }

        let mut cache = self.existing_folders_cache.write().unwrap();

        cache.insert(cdn_file.clone());

        Ok(())
    }
}
