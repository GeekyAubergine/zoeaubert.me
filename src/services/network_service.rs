use serde::de::DeserializeOwned;
use tracing::{debug, instrument};
use url::Url;

use crate::{
    domain::models::network_response::{
        NetworkResponse, NetworkResponseBody, NetworkResponseBodyJson, NetworkResponseBytes,
    },
    error::NetworkError,
    prelude::*,
};

#[derive(Debug)]
pub struct NetworkService2 {
    client: reqwest::Client,
}

impl NetworkService2 {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    #[instrument(
        skip_all,
        fields(method = "GET", url = %url),
        err
    )]
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

    #[instrument(
        skip_all,
        fields(method = "GET", url = %url),
        err
    )]
    pub async fn download_json<J>(&self, url: &Url) -> Result<J>
    where
        J: DeserializeOwned,
    {
        let resp = self.download_response(url).await?;

        let json = resp.json::<J>().await.map_err(NetworkError::fetch_error)?;

        Ok(json)
    }

    #[instrument(
        skip_all,
        fields(method = "GET", url = %url),
        err
    )]
    pub async fn download_bytes(&self, url: &Url) -> Result<Vec<u8>> {
        let resp = self.download_response(url).await?;

        let bytes = resp.bytes().await.map_err(NetworkError::fetch_error)?;

        Ok(bytes.to_vec())
    }
}
