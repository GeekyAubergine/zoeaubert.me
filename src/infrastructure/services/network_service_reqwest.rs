use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use crate::{
    domain::{
        models::network_response::{
            NetworkResponse, NetworkResponseBody, NetworkResponseBodyJson, NetworkResponseBytes,
        },
        services::NetworkService,
    },
    error::NetworkError,
    prelude::*,
};

pub struct NetworkServiceReqwest {
    client: reqwest::Client,
}

impl NetworkServiceReqwest {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn download_response(&self, url: &Url) -> Result<reqwest::Response> {
        debug!("Making request to: {}", url);

        let resp = self
            .client
            .get(url.as_str())
            .send()
            .await
            .map_err(NetworkError::fetch_error)?;

        Ok(resp)
    }
}

#[async_trait::async_trait]
impl NetworkService for NetworkServiceReqwest {
    async fn download_json<J>(&self, url: &Url) -> Result<J>
    where
        J: DeserializeOwned,
    {
        let resp = self.download_response(url).await?;

        let json = resp.json::<J>().await.map_err(NetworkError::fetch_error)?;

        Ok(json)
    }

    async fn download_bytes(&self, url: &Url) -> Result<Vec<u8>> {
        let resp = self.download_response(url).await?;

        let bytes = resp.bytes().await.map_err(NetworkError::fetch_error)?;

        Ok(bytes.to_vec())
    }
}
