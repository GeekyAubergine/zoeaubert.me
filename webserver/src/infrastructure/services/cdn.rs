use std::{fmt::Display, path::Path};

use dotenvy_macro::dotenv;
use reqwest::{header::ACCEPT, Body, ClientBuilder};
use serde::Deserialize;
use shared::cache::CachePath;
use tokio::fs::{self, File};
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::debug;

use crate::{error::Error, infrastructure::config::Config, prelude::*};

const CACHE_DIR: &str = dotenv!("CACHE_DIR");

#[derive(Debug, Clone, Deserialize)]
pub struct BunnyCdnFileResponse {
    #[serde(rename = "ObjectName")]
    object_name: String,
}

#[derive(Debug, Clone)]
pub struct CdnPath(String);

impl CdnPath {
    pub fn new(path: String) -> Self {
        Self(path.replace("//", "/"))
    }

    pub fn from_cache_path(path: &CachePath) -> Self {
        Self(path.as_str().replace(CACHE_DIR, ""))
    }

    pub fn file_name(&self) -> Result<&str> {
        self.0
            .split('/')
            .last()
            .ok_or(Error::CdnInvalidPath(self.to_string()))
    }

    pub fn parent_path(&self) -> Result<&str> {
        let file_name = self.file_name()?;

        self.0
            .strip_suffix(file_name)
            .ok_or(Error::CdnInvalidPath(self.to_string()))
    }

    pub fn extension(&self) -> Result<&str> {
        let file_name = self.file_name()?;

        file_name
            .split('.')
            .last()
            .ok_or(Error::CdnInvalidPath(self.to_string()))
    }

    pub fn file_name_without_extension(&self) -> Result<&str> {
        let file_name = self.file_name()?;

        file_name
            .split('.')
            .next()
            .ok_or(Error::CdnInvalidPath(self.to_string()))
    }

    pub fn path(&self) -> &str {
        &self.0
    }

    pub fn url(&self, config: &Config) -> String {
        format!("{}{}", config.cdn_url(), self.0)
    }
}

impl Display for CdnPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Cdn {
    reqwest_client: reqwest::Client,
}

impl Cdn {
    pub async fn new(config: &Config) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "AccessKey",
            config.bunny_cdn().access_key().parse().unwrap(),
        );

        Self {
            reqwest_client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn query_path(
        &self,
        path: &CdnPath,
        config: &Config,
    ) -> Result<Option<Vec<BunnyCdnFileResponse>>> {
        let request = self
            .reqwest_client
            .get(format!("{}{}", config.bunny_cdn().url(), path.path()))
            .header(ACCEPT, "application/json");

        match request.send().await {
            Ok(response) => {
                if response.status().as_u16() == 404 {
                    return Ok(None);
                }

                match response.json::<Vec<BunnyCdnFileResponse>>().await {
                    Ok(response) => Ok(Some(response)),
                    Err(e) => Err(Error::HttpReqwest(e)),
                }
            }
            Err(e) => Err(Error::HttpReqwest(e)),
        }
    }

    pub async fn files_names_in_folder(
        &self,
        folder: &CdnPath,
        config: &Config,
    ) -> Result<Vec<String>> {
        match self.query_path(folder, config).await {
            Ok(Some(files)) => Ok(files.iter().map(|f| f.object_name.clone()).collect()),
            Ok(None) => Ok(vec![]),
            Err(e) => Err(e),
        }
    }

    pub async fn file_exists(&self, path: &CdnPath, config: &Config) -> Result<bool> {
        let filename = path.file_name()?;
        let path_without_filename = path.parent_path()?;

        let files = self
            .files_names_in_folder(&CdnPath::new(format!("{}/", path_without_filename)), config)
            .await?;

        Ok(files.contains(&filename.to_string()))
    }

    pub async fn upload_file_from_cache(
        &self,
        cache_path: &CachePath,
        cnd_path: &CdnPath,
        config: &Config,
    ) -> Result<()> {
        let file = File::open(cache_path.as_path())
            .await
            .map_err(Error::FileSystemUnreadable)?;

        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);

        // let file = fs::read(cache_path.path())
        //     .await
        //     .map_err(Error::FileSystemUnreadable)?;

        let request = self
            .reqwest_client
            .put(format!("{}{}", config.bunny_cdn().url(), cnd_path.path()))
            .header("Content-Type", "application/octet-stream")
            .body(file_body);

        let response = request.send().await.map_err(Error::CdnUpload)?;

        if response.status().as_u16() != 201 {
            return Err(Error::CdnUnableToUploadFile(
                cache_path.to_string(),
                cnd_path.to_string(),
            ));
        }

        Ok(())
    }

    pub async fn upload_file_from_bytes(
        &self,
        data: Vec<u8>,
        cnd_path: &CdnPath,
        config: &Config,
    ) -> Result<()> {
        let request = self
            .reqwest_client
            .put(format!("{}{}", config.bunny_cdn().url(), cnd_path.path()))
            .header("Content-Type", "application/octet-stream")
            .body(data);

        let response = request.send().await.map_err(Error::CdnUpload)?;

        if response.status().as_u16() != 201 {
            return Err(Error::CdnUnableToUploadFile(
                "bytes".to_string(),
                cnd_path.to_string(),
            ));
        }

        Ok(())
    }

    pub async fn download_file(&self, path: &CdnPath, config: &Config) -> Result<Vec<u8>> {
        let request = self
            .reqwest_client
            .get(format!("{}{}", config.bunny_cdn().url(), path.path()))
            .header(ACCEPT, "*/*");

        let response = request.send().await.map_err(Error::CdnDownload)?;

        if response.status().as_u16() == 404 {
            return Err(Error::CdnFileNotFound(path.to_string()));
        }

        let bytes = response.bytes().await.map_err(Error::CdnDownload)?;

        Ok(bytes.to_vec())
    }
}
