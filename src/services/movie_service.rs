use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chrono::Datelike;
use htmlentity::entity::{ICodedDataTrait, decode};
use serde::Deserialize;
use tracing::{instrument, warn};
use url::Url;

use crate::config::CONFIG;
use crate::domain::models::movie::{Movie, MovieId};
use crate::error::MovieError;
use crate::prelude::*;

use crate::services::ServiceContext;
use crate::services::cdn_service::CdnFile;
use crate::services::file_service::{ArchiveFile, FileService, ReadableFile, WritableFile};
use crate::services::media_service::MediaService;
use crate::utils::date::parse_date;

const FILE_NAME: &str = "movie_cache.json";

const TMDB_LINK_URL: &str = "https://www.themoviedb.org/movie/";
const TMDB_IMAGE_URL: &str = "https://image.tmdb.org/t/p/w200";

fn name_and_year_to_key(name: &str, year: u16) -> String {
    format!("{} ({})", name, year)
}

fn make_search_url(title: &str, year: u16) -> Url {
    let title = decode(title.as_bytes()).to_string().unwrap();

    let title = title.replace('&', "").replace(' ', "+");

    format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&year={}",
        CONFIG.tmdb.key, title, year
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
    results: Vec<TmdbSearchResponseSingle>,
}

#[derive(Debug)]
pub struct MovieService {
    file: ArchiveFile,
    movies: Arc<RwLock<HashMap<String, Option<Movie>>>>,
}

impl MovieService {
    pub fn new() -> Result<Self> {
        let file = FileService::archive(FILE_NAME.into());

        let data = file.read_json_or_default()?;

        Ok(Self {
            file,
            movies: Arc::new(RwLock::new(data)),
        })
    }

    #[instrument(err, skip_all, fields(movie.title=%title, movie.year=&year))]
    pub fn find_movie(
        &self,
        ctx: &ServiceContext,
        title: &str,
        year: u16,
    ) -> Result<Option<Movie>> {
        let key = name_and_year_to_key(title, year);

        let mut movies = self.movies.write().unwrap();

        if let Some(movie) = movies.get(&key) {
            match movie {
                Some(movie) => return Ok(Some(movie.clone())),
                None => {
                    warn!("Did not find cover for movie [{title} - {year}]");
                    return Ok(None);
                }
            }
        }

        let results = ctx
            .network
            .download_json::<TmdbSearchResponse>(&make_search_url(title, year))?;

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

                let image_url = &format!("{}{}", TMDB_IMAGE_URL, poster).parse().unwrap();

                let cdn_file = CdnFile::from_str(&format!("movies/{}-poster-200.jpg", movie.id));

                let image = MediaService::image_from_url(
                    ctx,
                    image_url,
                    &cdn_file,
                    &format!("{} movie poster", movie.title),
                    Some(&format!("https://www.themoviedb.org/movie/{}", movie.id)),
                    None,
                )?;

                let date = parse_date(&movie.release_date)?;

                let movie = Movie {
                    title: movie.title.clone(),
                    year: date.year() as u16,
                    poster: image,
                    id: MovieId::tmdb(movie.id),
                    link: format!("{}{}", TMDB_LINK_URL, movie.id).parse().unwrap(),
                };

                movies.insert(key, Some(movie.clone()));

                self.file.write_json(&movies.clone())?;

                Ok(Some(movie))
            }
            None => {
                warn!("Did not find cover for movie [{title}]");
                movies.insert(title.to_string(), None);

                self.file.write_json(&movies.clone())?;

                Ok(None)
            }
        }
    }
}
