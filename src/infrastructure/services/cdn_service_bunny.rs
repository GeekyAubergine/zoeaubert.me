use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use reqwest::header::ACCEPT;
use reqwest::{Body, ClientBuilder};
use serde::Deserialize;
use url::Url;
use std::path::PathBuf;
use std::sync::{Arc};
use std::{fs::read_dir, path::Path};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::debug;
use tokio::sync::RwLock;

use crate::domain::models::cache_path::CachePath;
use crate::domain::services::{CacheService, CdnService};
use crate::domain::state::State;
use crate::error::{CdnError, FileSystemError, NetworkError};
use crate::infrastructure::utils::file_system::make_cache_file_path;
use crate::prelude::*;

fn make_cdn_api_url(path: &str) -> String {
    format!("{}/{}", dotenv!("BUNNY_CDN_URL"), path)
}

#[derive(Debug, Clone, Deserialize)]
pub struct BunnyCdnFileResponse {
    #[serde(rename = "ObjectName")]
    object_name: String,
}

#[derive(Debug, Clone)]
pub struct CopyFileJob {
    source: Url,
    destination: PathBuf,
}

pub struct CdnServiceBunny {
    reqwest_client: reqwest::Client,
    copy_file_jobs: Arc<RwLock<Vec<CopyFileJob>>>,
}

impl CdnServiceBunny {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "AccessKey",
            dotenv!("BUNNY_CDN_ACCESS_KEY").parse().unwrap(),
        );

        Self {
            reqwest_client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
            copy_file_jobs: Arc::new(RwLock::new(vec![])),
        }
    }

    async fn query_path(&self, path: &Path) -> Result<Option<Vec<BunnyCdnFileResponse>>> {
        let path = match path.is_file() {
            true => path.to_string_lossy().to_string(),
            false => format!("{}/", path.to_string_lossy()),
        };

        debug!("Querying path in cdn: {}", path);
        let request = self
            .reqwest_client
            .get(make_cdn_api_url(&path))
            .header(ACCEPT, "application/json");

        match request.send().await {
            Ok(response) => {
                if response.status().as_u16() == 404 {
                    return Ok(None);
                }

                match response.json::<Vec<BunnyCdnFileResponse>>().await {
                    Ok(response) => Ok(Some(response)),
                    Err(e) => Err(NetworkError::fetch_error(e)),
                }
            }
            Err(e) => Err(NetworkError::fetch_error(e)),
        }
    }

    async fn files_names_in_folder(&self, path: &Path) -> Result<Vec<String>> {
        let folder_path = match path.is_file() {
            true => path.parent().unwrap(),
            false => path,
        };

        match self.query_path(&folder_path).await {
            Ok(Some(files)) => Ok(files.iter().map(|f| f.object_name.clone()).collect()),
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    async fn file_exists(&self, file_name: &Path) -> Result<bool> {
        debug!("Checking if file exists in cdn: {:?}", file_name);

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

        debug!("File opened");

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

        Ok(())
    }

    async fn copy_file_from_url_to_cdn(
        &self,
        state: &impl State,
        source: &Url,
        destination: &Path,
    ) -> Result<()> {
        let mut jobs = self.copy_file_jobs.write().await;

        jobs.push(CopyFileJob {
            source: source.clone(),
            destination: destination.into(),
        });

        Ok(())
    }

    async fn process_queue(&self) -> Result<()> {
        // TODO

        Ok(())
    }
}
