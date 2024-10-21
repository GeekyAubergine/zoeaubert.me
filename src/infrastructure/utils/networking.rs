use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use crate::domain::services::CacheService;
use crate::domain::services::CdnService;
use crate::domain::state::State;
use crate::error::*;
use crate::prelude::*;

pub async fn download_response(client: &reqwest::Client, url: &Url) -> Result<reqwest::Response> {
    debug!("Making request to: {}", url);

    let resp = client
        .get(url.as_str())
        .send()
        .await
        .map_err(NetworkError::fetch_error)?;

    Ok(resp)
}

pub async fn download_json<T>(client: &reqwest::Client, url: &Url) -> Result<T>
where
    T: DeserializeOwned,
{
    let resp = download_response(client, url).await?;

    let json = resp.json::<T>().await.map_err(NetworkError::fetch_error)?;

    Ok(json)
}

pub async fn download_bytes(client: &reqwest::Client, url: &Url) -> Result<Vec<u8>> {
    let resp = download_response(client, url).await?;

    let bytes = resp.bytes().await.map_err(NetworkError::fetch_error)?;

    Ok(bytes.to_vec())
}

// pub async fn copy_file_from_url_to_cdn(
//     state: &impl State,
//     source: &str,
//     destiniation: &str,
// ) -> Result<()> {
//     state
//         .cache_service()
//         .download_and_cache_file(&source)
//         .await?;

//     state
//         .cdn_service()
//         .upload_file(&source, &destiniation)
//         .await
// }
