use std::path::Path;

use crate::{domain::{services::CdnService, state::State}, error::ImageError};
use imagesize::blob_size;
use url::Url;

use crate::{
    domain::{models::image::Image, services::CacheService},
    prelude::*,
};

pub async fn image_from_url(
    state: &impl State,
    url: &Url,
    path: &Path,
    alt: &str,
) -> Result<Image> {
    let data = state
        .cache_service()
        .get_file_from_cache_or_url(url)
        .await?;

    let image_size = blob_size(&data).map_err(ImageError::size_error)?;

    state
        .cdn_service()
        .copy_file_from_url_to_cdn(state, &url, &path)
        .await?;

    Ok(Image::new(
        path,
        alt,
        image_size.width as u32,
        image_size.height as u32,
    ))
}
