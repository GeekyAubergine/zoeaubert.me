use aws_config::{BehaviorVersion, Region};
use aws_credential_types::Credentials;
use aws_sdk_s3::{primitives::ByteStream, types::ObjectCannedAcl, Client};

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
    r2_client: Client,
}

impl Cdn {
    pub async fn new(config: &Config) -> Self {
        let credentials = Credentials::from_keys(config.r2().key(), config.r2().secret(), None);

        let r2_config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region(Region::new("auto"))
            .endpoint_url(config.r2().endpoint().to_owned())
            .credentials_provider(credentials)
            .load()
            .await;

        let r2_client = aws_sdk_s3::Client::new(&r2_config);

        Self {
            reqwest_client: reqwest::Client::new(),
            r2_client,
        }
    }

    pub fn reqwest_client(&self) -> &reqwest::Client {
        &self.reqwest_client
    }

    pub async fn file_exists(&self, path: CndPath, config: &Config) -> Result<bool> {
        let request = self
            .r2_client
            .head_object()
            .bucket(config.r2().bucket())
            .key(path.path.to_owned());

        match request.send().await {
            Ok(_) => Ok(true),
            Err(e) => Ok(false),
        }
    }

    pub async fn upload_file(&self, path: CndPath, data: Vec<u8>, config: &Config) -> Result<()> {
        let data = ByteStream::from(data);

        let request = self
            .r2_client
            .put_object()
            .bucket(config.r2().bucket())
            .key(path.path)
            .acl(ObjectCannedAcl::PublicRead)
            .body(data);

        request.send().await.map_err(Error::CdnUpload)?;

        Ok(())
    }
}
