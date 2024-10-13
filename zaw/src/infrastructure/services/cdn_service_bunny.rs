use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use reqwest::header::ACCEPT;
use reqwest::{Body, ClientBuilder};
use serde::Deserialize;
use std::{fs::read_dir, path::Path};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::debug;

use crate::domain::models::cache_path::CachePath;
use crate::domain::services::{CacheService, CdnService};
use crate::domain::state::State;
use crate::error::{CdnError, FileSystemError, NetworkError};
use crate::infrastructure::utils::file_system::make_cache_file_path;
use crate::prelude::*;

pub fn make_cdn_path_for_file_with_date(
    date: &DateTime<Utc>,
    file_name: &str,
    suffix: Option<&str>,
) -> String {
    let date_str = date.format("%Y/%m/%d").to_string();

    let file_name = file_name.split('/').last().unwrap();

    let name = file_name.split('.').next().unwrap();
    let ext = file_name.split('.').last().unwrap();

    let suffix_str = match suffix {
        Some(suffix) => format!("-{}", suffix),
        None => "".to_string(),
    };

    format!("{}/{}{}.{}", date_str, name, suffix_str, ext)
}

pub fn make_cdn_url(url: &str) -> String {
    format!("{}/{}", dotenv!("CDN_URL"), url)
}

fn make_cdn_path(path: &str) -> String {
    format!("{}{}", dotenv!("BUNNY_CDN_URL"), path)
}

#[derive(Debug, Clone, Deserialize)]
pub struct BunnyCdnFileResponse {
    #[serde(rename = "ObjectName")]
    object_name: String,
}

pub struct CdnServiceBunny {
    reqwest_client: reqwest::Client,
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
        }
    }

    pub async fn query_path(&self, path: &str) -> Result<Option<Vec<BunnyCdnFileResponse>>> {
        debug!("Querying path in cdn: {}", path);
        let request = self
            .reqwest_client
            .get(make_cdn_path(path))
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

    pub async fn files_names_in_folder(&self, folder: &str) -> Result<Vec<String>> {
        match self.query_path(folder).await {
            Ok(Some(files)) => Ok(files.iter().map(|f| f.object_name.clone()).collect()),
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub async fn file_exists(&self, file_name: &str) -> Result<bool> {
        debug!("Checking if file exists in cdn: {}", file_name);

        let folder_name = Path::new(file_name).parent().unwrap().to_str().unwrap();

        let folder_name = format!("{}/", folder_name);

        let files = self.files_names_in_folder(&folder_name).await?;

        Ok(files.contains(&file_name.to_string()))
    }
}

#[async_trait::async_trait]
impl CdnService for CdnServiceBunny {
    async fn upload_file(&self, source: &str, destination: &str) -> Result<()> {
        debug!("Copying file from {} to {}", source, destination);
        if self.file_exists(destination).await? {
            debug!("File already exists in destination, skipping");
            return Ok(());
        }

        debug!("File does not exist in destination, copying");

        let cache_path = CachePath::from_url(source);

        debug!("Cache path: {:?}", cache_path);

        let file = File::open(&make_cache_file_path(cache_path.as_str()))
            .await
            .map_err(FileSystemError::read_error)?;

        debug!("File opened");

        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);

        let request = self
            .reqwest_client
            .put(make_cdn_path(destination))
            .header("Content-Type", "application/octet-stream")
            .body(file_body);

        let response = request.send().await.map_err(CdnError::upload_error)?;

        if response.status().as_u16() != 201 {
            return Err(CdnError::base_status(response.status().as_u16()));
        }

        Ok(())
    }
}
