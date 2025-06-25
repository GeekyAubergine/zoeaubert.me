use chrono::{DateTime, Datelike, Utc};
use dotenvy_macro::dotenv;
use htmlentity::entity::{decode, ICodedDataTrait};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

use serde::{Deserialize, Serialize};

use crate::domain::models::raw_content::SourcePost;
use crate::domain::models::omni_post::Post;
use crate::domain::models::tv_show::{TvShow, TvShowId, TvShowReview};
use crate::domain::services::{FileService, ImageService, NetworkService, TvShowsService};
use crate::domain::state::State;

use crate::error::TvShowsError;
use crate::infrastructure::utils::date::parse_date;
use crate::infrastructure::utils::parse_omni_post_content_into_movie_review::parse_content_into_movie_review;
use crate::infrastructure::utils::parse_omni_post_into_tv_show_reviews::{
    self, parse_content_into_tv_show_review,
};
use crate::prelude::*;

use super::file_service_disk::FileServiceDisk;

const FILE_NAME: &str = "tv_shows_cache.json";
const TMDB_LINK_URL: &str = "https://www.themoviedb.org/tv/";
const TMDB_IMAGE_URL: &str = "https://image.tmdb.org/t/p/w200";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

fn make_search_url(title: &str) -> Url {
    let title = decode(title.as_bytes()).to_string().unwrap();

    let title = title.replace('&', "").replace(' ', "+");

    format!(
        "https://api.themoviedb.org/3/search/tv?api_key={}&query={}",
        dotenv!("TMDB_KEY"),
        title,
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Deserialize)]
struct TmdbSearchResponseSingle {
    id: u32,
    name: String,
    poster_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TmdbSearchResponse {
    page: u32,
    results: Vec<TmdbSearchResponseSingle>,
    total_pages: u32,
    total_results: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SearchCacheData {
    movies: HashMap<String, TvShow>,
}

pub struct TvShowsServiceTmdb {
    client: reqwest::Client,
    data: Arc<RwLock<SearchCacheData>>,
    file_service: FileServiceDisk,
}

impl TvShowsServiceTmdb {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            client: reqwest::Client::new(),
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl TvShowsService for TvShowsServiceTmdb {
    async fn find_tv_show(&self, state: &impl State, title: &str) -> Result<Option<TvShow>> {
        if let Some(tv_show) = self.data.read().await.movies.get(title) {
            return Ok(Some(tv_show.clone()));
        }

        let results = state
            .network_service()
            .download_json::<TmdbSearchResponse>(&make_search_url(title))
            .await?;

        let results = results
            .results
            .iter()
            .filter(|r| r.poster_path.is_some())
            .collect::<Vec<_>>();

        match results.first() {
            Some(tv_show) => {
                let poster = tv_show
                    .poster_path
                    .as_ref()
                    .ok_or(TvShowsError::tv_show_has_no_poster(tv_show.id))?;

                let image_url = format!("{}{}", TMDB_IMAGE_URL, poster).parse().unwrap();

                let image_path = &format!("tv/{}-poster-400.jpg", tv_show.id);
                let image_path = Path::new(&image_path);

                let image = state
                    .image_service()
                    .copy_image_from_url(
                        state,
                        &image_url,
                        &image_path,
                        &format!("{} movie poster", tv_show.name),
                    )
                    .await?;

                println!("Image: {:?}", image);

                let tv_show = TvShow {
                    title: tv_show.name.clone(),
                    poster: image,
                    id: TvShowId::tmdb(tv_show.id),
                    link: format!("{}{}", TMDB_LINK_URL, tv_show.id).parse().unwrap(),
                };

                let mut data = self.data.write().await;
                data.movies.insert(title.to_string(), tv_show.clone());

                self.file_service
                    .write_json_file(&make_file_path(&self.file_service), &data.clone())
                    .await?;

                Ok(Some(tv_show))
            }
            None => Ok(None),
        }
    }

    async fn tv_show_review_from_content(
        &self,
        state: &impl State,
        post: &SourcePost,
    ) -> Result<TvShowReview> {
        let review = parse_content_into_tv_show_review(post)?;

        let tv_show = self
            .find_tv_show(state, &review.title)
            .await?
            .ok_or(TvShowsError::tv_show_not_found(review.title))?;

        Ok(TvShowReview {
            tv_show,
            seasons: review.seasons,
            scores: review.scores,
            review: review.review,
            source_content: post.clone(),
        })
    }
}
