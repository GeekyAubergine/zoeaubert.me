use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use crate::{
    domain::models::network_response::{
        NetworkResponse, NetworkResponseBody, NetworkResponseBodyJson, NetworkResponseBytes,
    },
    error::NetworkError,
    prelude::*,
};

pub struct NetworkService2 {
    client: reqwest::Client,
}

impl NetworkService2 {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_response(&self, url: &Url) -> Result<reqwest::Response> {
        debug!("Making request to: {}", url);

        let resp = self
            .client
            .get(url.as_str())
            .send()
            .await
            .map_err(NetworkError::fetch_error)?;

        Ok(resp)
    }

    pub async fn download_json<J>(&self, url: &Url) -> Result<J>
    where
        J: DeserializeOwned,
    {
        let resp = self.download_response(url).await?;

        let json = resp.json::<J>().await.map_err(NetworkError::fetch_error)?;

        Ok(json)
    }

    pub async fn download_bytes(&self, url: &Url) -> Result<Vec<u8>> {
        let resp = self.download_response(url).await?;

        let bytes = resp.bytes().await.map_err(NetworkError::fetch_error)?;

        Ok(bytes.to_vec())
    }
}
