use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use reqwest::header::ACCEPT;
use reqwest::{Body, ClientBuilder};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::{fs::read_dir, path::Path};
use tokio::fs::File;
use tokio::sync::RwLock;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::debug;
use url::Url;

use crate::domain::models::cache_path::CachePath;
use crate::domain::services::{CacheService, CdnService};
use crate::domain::state::State;
use crate::error::{CdnError, FileSystemError, NetworkError};
use crate::infrastructure::utils::file_system::make_cache_file_path;
use crate::infrastructure::utils::networking::{download_json, download_response};
use crate::prelude::*;

fn make_cdn_api_url(path: &str) -> Url {
    format!("{}{}", dotenv!("BUNNY_CDN_URL"), path)
        .parse()
        .unwrap()
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
        let removable_prefix = format!("/{}", self.storage_zone_name);
        path.replace(&removable_prefix, "")
    }
}

pub struct CdnServiceBunny {
    reqwest_client: reqwest::Client,
    existing_folders_cache: Arc<RwLock<HashSet<String>>>,
}

impl CdnServiceBunny {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "AccessKey",
            dotenv!("BUNNY_CDN_ACCESS_KEY").parse().unwrap(),
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

    async fn query_path(&self, path: &Path) -> Result<Option<Vec<BunnyCdnFileResponse>>> {
        let path = match path.to_string_lossy().ends_with("/") {
            true => path.to_string_lossy().into_owned(),
            false => format!("{}/", path.to_string_lossy()),
        };

        debug!("Querying path in cdn: {:?}", path);

        let response = download_response(
            &self.reqwest_client,
            &make_cdn_api_url(&path),
        )
        .await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        match response.json::<Vec<BunnyCdnFileResponse>>().await {
            Ok(response) => Ok(Some(response)),
            Err(e) => Err(NetworkError::fetch_error(e)),
        }
    }

    async fn files_names_in_folder(&self, path: &Path) -> Result<Vec<String>> {
        debug!(
            "Querying files in folder: {:?} {:?}",
            path.to_string_lossy(),
            path.is_dir()
        );

        let folder_path = match path.is_dir() {
            true => path,
            false => path.parent().unwrap(),
        };

        match self.query_path(&folder_path).await {
            Ok(Some(files)) => {
                let mut cache = self.existing_folders_cache.write().await;

                for file in files.iter() {
                    cache.insert(file.path());
                }

                Ok(files.iter().map(|f| f.object_name.clone()).collect())
            }
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    async fn file_exists(&self, file_name: &Path) -> Result<bool> {
        let cache_path = match file_name.to_string_lossy().starts_with("/") {
            true => file_name.to_string_lossy().into_owned(),
            false => format!("/{}", file_name.to_string_lossy()),
        };

        if let Some(file) = self
            .existing_folders_cache
            .read()
            .await
            .get(&cache_path)
        {
            debug!("File exists in cache: {:?}", file);
            return Ok(true);
        }

        let files = self.files_names_in_folder(&file_name).await?;

        let file_name = file_name.file_name().unwrap().to_string_lossy().into();

        Ok(files.contains(&file_name))
    }
}

#[async_trait::async_trait]
impl CdnService for CdnServiceBunny {
    async fn upload_file(&self, source: &Path, destination: &Path) -> Result<()> {
        debug!("Copying file from {:?} to {:?}", source, destination);
        if self.file_exists(destination).await? {
            debug!("File already exists in destination, skipping");
            return Ok(());
        }

        debug!("File does not exist in destination, copying");

        let file = File::open(&make_cache_file_path(source))
            .await
            .map_err(FileSystemError::read_error)?;

        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);

        let request = self
            .reqwest_client
            .put(make_cdn_api_url(&destination.to_string_lossy().to_string()))
            .header("Content-Type", "application/octet-stream")
            .body(file_body);

        let response = request.send().await.map_err(CdnError::upload_error)?;

        if response.status().as_u16() != 201 {
            return Err(CdnError::base_status(response.status().as_u16()));
        }

        let mut cache = self.existing_folders_cache.write().await;

        cache.insert(destination.to_string_lossy().to_string());

        Ok(())
    }
}
