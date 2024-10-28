use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use super::{
    models::{
        cache_path::CachePath,
        image::{Image, ImageDimensions},
        media::Media,
        movie::{Movie, MovieReview},
        network_response::{NetworkResponse, NetworkResponseBodyJson, NetworkResponseBytes},
        omni_post::OmniPost,
        slug::Slug,
        tv_show::{TvShow, TvShowReview},
    },
    state::State,
};

use crate::prelude::*;

#[async_trait::async_trait]
pub trait CacheService {
    async fn is_file_cached(&self, path: &Path) -> Result<bool>;

    async fn read_file(&self, state: &impl State, path: &Path) -> Result<Vec<u8>>;

    async fn write_file(&self, state: &impl State, path: &Path, content: &[u8]) -> Result<()>;
}

#[async_trait::async_trait]
pub trait CdnService {
    async fn upload_file(
        &self,
        state: &impl State,
        source: &Path,
        destination: &Path,
    ) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MovieService {
    async fn find_movie(&self, state: &impl State, title: &str, year: u16)
        -> Result<Option<Movie>>;

    async fn movie_review_from_omni_post(
        &self,
        state: &impl State,
        post: &OmniPost,
    ) -> Result<MovieReview>;
}

#[async_trait::async_trait]
pub trait TvShowsService {
    async fn find_tv_show(&self, state: &impl State, title: &str) -> Result<Option<TvShow>>;

    async fn tv_show_review_from_omni_post(
        &self,
        state: &impl State,
        post: &OmniPost,
    ) -> Result<TvShowReview>;
}

#[async_trait::async_trait]
pub trait ImageService {
    async fn copy_image_from_url(
        &self,
        state: &impl State,
        url: &Url,
        path: &Path,
        alt: &str,
    ) -> Result<Image>;

    async fn find_images_in_markdown(
        &self,
        state: &impl State,
        markdown: &str,
        date: &DateTime<Utc>,
        parent_slug: &Slug,
    ) -> Result<Vec<Image>>;

    async fn copy_and_resize_image_from_url(
        &self,
        state: &impl State,
        url: &Url,
        path: &Path,
        alt: &str,
        new_size: &ImageDimensions
    ) -> Result<Image>;
}

#[async_trait::async_trait]
pub trait NetworkService {
    async fn download_json<J>(&self, url: &Url) -> Result<J>
    where
        J: DeserializeOwned;

    async fn download_bytes(&self, url: &Url) -> Result<Vec<u8>>;
}

#[async_trait::async_trait]
pub trait FileService: Sized + Send + Sync {
    fn make_archive_file_path(&self, path: &Path) -> PathBuf;

    fn make_content_file_path(&self, path: &Path) -> PathBuf;

    fn make_cache_file_path(&self, path: &Path) -> PathBuf;

    fn make_output_file_path(&self, path: &Path) -> PathBuf;

    fn make_file_path_from_date_and_file(
        &self,
        date: &DateTime<Utc>,
        file_name: &str,
        suffix: Option<&str>,
    ) -> PathBuf;

    async fn find_files_rescurse(&self, path: &Path, extension: &str) -> Result<Vec<String>>;

    async fn get_file_metadata(&self, path: &Path) -> Result<std::fs::Metadata>;

    async fn get_file_last_modified(&self, path: &Path) -> Result<DateTime<Utc>>;

    async fn read_file(&self, path: &Path) -> Result<Vec<u8>>;

    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()>;

    async fn read_json_file<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned;

    async fn read_json_file_or_default<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned + Default;

    async fn write_json_file<D>(&self, path: &Path, data: &D) -> Result<()>
    where
        D: Serialize + Send + Sync;

    async fn read_csv_file<D>(&self, path: &Path) -> Result<Vec<D>>
    where
        D: DeserializeOwned;

    async fn read_text_file(&self, path: &Path) -> Result<String>;

    async fn write_text_file_blocking(&self, path: &Path, data: &str) -> Result<()>;

    async fn write_text_file(&self, path: PathBuf, data: String) -> Result<()>;

    async fn read_yaml_file<D>(&self, path: &Path) -> Result<D>
    where
        D: DeserializeOwned;
}

#[async_trait::async_trait]
pub trait QueryLimitingService {
    async fn can_query(&self, query: &str, no_query_duration: &Duration) -> Result<bool>;

    async fn can_query_within_hour(&self, query: &str) -> Result<bool>;

    async fn can_query_within_day(&self, query: &str) -> Result<bool>;
}
