use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{primitives::ByteStream, types::ObjectCannedAcl, Client};
use reqwest::{header::ACCEPT, Body, ClientBuilder};

use crate::{error::Error, prelude::*};

use super::config::Config;

#[derive(Debug, Clone)]
pub struct CndPath {
    path: String,
}

impl CndPath {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[derive(Debug, Clone)]
pub struct Cdn {
    reqwest_client: reqwest::Client,
}

impl Cdn {
    pub async fn new(config: &Config) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
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

    pub async fn file_exists(&self, path: CndPath, config: &Config) -> Result<bool> {
        let request =
            self.reqwest_client
                .get(format!("{}/{}", config.bunny_cdn().url(), path.path));

        match request.send().await {
            Ok(_) => Ok(true),
            Err(e) => Ok(false),
        }
    }

    pub async fn upload_file(&self, path: CndPath, data: Vec<u8>, config: &Config) -> Result<()> {
        let data = Body::from(data);

        let request = self
            .reqwest_client
            .post(format!("{}/{}", config.bunny_cdn().url(), path.path))
            .header("Content-Type", "application/octet-stream")
            .body(data);

        request.send().await.map_err(Error::CdnUpload)?;

        Ok(())
    }

    pub async fn download_file(&self, path: CndPath, config: &Config) -> Result<Vec<u8>> {
        let request = self
            .reqwest_client
            .get(format!("{}/{}", config.bunny_cdn().url(), path.path))
            .header(ACCEPT, "*/*");

        let response = request.send().await.map_err(Error::CdnDownload)?;

        let bytes = response.bytes().await.map_err(Error::CdnDownload)?;

        Ok(bytes.to_vec())
    }
}
