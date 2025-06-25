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
use crate::domain::models::movie::{MovieId, MovieReview};
use crate::domain::models::omni_post::Post;
use crate::domain::services::{FileService, ImageService, NetworkService};
use crate::domain::state::State;
use crate::domain::{models::movie::Movie, services::MovieService};

use crate::error::MovieError;
use crate::infrastructure::utils::date::parse_date;
use crate::infrastructure::utils::parse_omni_post_content_into_movie_review::parse_content_into_movie_review;
use crate::prelude::*;

use super::file_service_disk::FileServiceDisk;

const FILE_NAME: &str = "movie_cache.json";
const TMDB_LINK_URL: &str = "https://www.themoviedb.org/movie/";
const TMDB_IMAGE_URL: &str = "https://image.tmdb.org/t/p/w200";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

fn name_and_year_to_key(name: &str, year: u16) -> String {
    format!("{} ({})", name, year)
}

fn make_search_url(title: &str, year: u16) -> Url {
    let title = decode(title.as_bytes()).to_string().unwrap();

    let title = title.replace('&', "").replace(' ', "+");

    format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&year={}",
        dotenv!("TMDB_KEY"),
        title,
        year
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Deserialize)]
struct TmdbSearchResponseSingle {
    id: u32,
    title: String,
    poster_path: Option<String>,
    release_date: String,
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
    movies: HashMap<String, Movie>,
}

pub struct MovieServiceTmdb {
    client: reqwest::Client,
    data: Arc<RwLock<SearchCacheData>>,
    file_service: FileServiceDisk,
}

impl MovieServiceTmdb {
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
impl MovieService for MovieServiceTmdb {
    async fn find_movie(
        &self,
        state: &impl State,
        title: &str,
        year: u16,
    ) -> Result<Option<Movie>> {
        let key = name_and_year_to_key(title, year);

        if let Some(movie) = self.data.read().await.movies.get(&key) {
            return Ok(Some(movie.clone()));
        }

        let results = state
            .network_service()
            .download_json::<TmdbSearchResponse>(&make_search_url(title, year))
            .await?;

        let results = results
            .results
            .iter()
            .filter(|r| r.poster_path.is_some())
            .collect::<Vec<_>>();

        match results.first() {
            Some(movie) => {
                let poster = movie
                    .poster_path
                    .as_ref()
                    .ok_or(MovieError::movie_has_no_poster(movie.id))?;

                let image_url = format!("{}{}", TMDB_IMAGE_URL, poster).parse().unwrap();

                let image_path = &format!("movies/{}-poster-200.jpg", movie.id);
                let image_path = Path::new(&image_path);

                let image = state
                    .image_service()
                    .copy_image_from_url(
                        state,
                        &image_url,
                        &image_path,
                        &format!("{} movie poster", movie.title),
                    )
                    .await?;

                let date = parse_date(&movie.release_date)?;

                let movie = Movie {
                    title: movie.title.clone(),
                    year: date.year() as u16,
                    poster: image,
                    id: MovieId::tmdb(movie.id),
                    link: format!("{}{}", TMDB_LINK_URL, movie.id).parse().unwrap(),
                };

                let mut data = self.data.write().await;
                data.movies.insert(key, movie.clone());

                self.file_service
                    .write_json_file(&make_file_path(&self.file_service), &data.clone())
                    .await?;

                Ok(Some(movie))
            }
            None => Ok(None),
        }
    }

    async fn movie_review_from_content(
        &self,
        state: &impl State,
        post: &SourcePost,
    ) -> Result<MovieReview> {
        let review = parse_content_into_movie_review(post)?;

        let movie = self
            .find_movie(state, &review.title, review.year)
            .await?
            .ok_or(MovieError::movie_not_found(review.title))?;

        Ok(MovieReview {
            movie,
            score: review.score,
            review: review.review,
            source_content: post.clone(),
        })
    }
}
