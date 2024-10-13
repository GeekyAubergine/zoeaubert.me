use serde::de::DeserializeOwned;
use tracing::debug;

use crate::error::*;
use crate::prelude::*;

async fn download_response(client: &reqwest::Client, url: &str) -> Result<reqwest::Response> {
    debug!("Making request to: {}", url);

    let resp = client.get(url).send().await.map_err(NetworkError::fetch_error)?;

    Ok(resp)
}

pub async fn download_json<T>(client: &reqwest::Client, url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let resp = download_response(client, url).await?;

    let json = resp.json::<T>().await.map_err(NetworkError::fetch_error)?;

    Ok(json)
}

pub async fn download_bytes(client: &reqwest::Client, url: &str) -> Result<Vec<u8>> {
    let resp = download_response(client, url).await?;

    let bytes = resp.bytes().await.map_err(NetworkError::fetch_error)?;

    Ok(bytes.to_vec())
}
