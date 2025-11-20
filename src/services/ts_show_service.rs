use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use chrono::Datelike;
use htmlentity::entity::{ICodedDataTrait, decode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};
use url::Url;

use dotenvy_macro::dotenv;

use crate::domain::models::book::{Book, BookID};
use crate::domain::models::movie::{Movie, MovieId};
use crate::domain::models::slug::Slug;
use crate::domain::models::tv_show::{TvShow, TvShowId};
use crate::error::{BookError, MovieError, TvShowsError};
use crate::prelude::*;

use crate::services::cdn_service::CdnFile;
use crate::services::file_service::{ArchiveFile, FileService, ReadableFile, WritableFile};
use crate::services::media_service::MediaService;
use crate::utils::date::parse_date;
use crate::{domain::models::tag::Tag, services::ServiceContext};

const FILE_NAME: &str = "tv_shows_cache.json";
const TMDB_LINK_URL: &str = "https://www.themoviedb.org/tv/";
const TMDB_IMAGE_URL: &str = "https://image.tmdb.org/t/p/w200";

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

#[derive(Debug)]
pub struct TvShowService {
    file: ArchiveFile,
    tv_shows: Arc<RwLock<HashMap<String, Option<TvShow>>>>,
}

impl TvShowService {
    pub fn new() -> Result<Self> {
        let file = FileService::archive(FILE_NAME.into());

        let data = file.read_json_or_default()?;

        Ok(Self {
            file,
            tv_shows: Arc::new(RwLock::new(data)),
        })
    }

    #[instrument(err, skip_all, fields(tv_show.title=%title))]
    pub async fn find_tv_show(&self, ctx: &ServiceContext, title: &str) -> Result<Option<TvShow>> {
        let mut tv_shows = self.tv_shows.write().unwrap();

        if let Some(movie) = tv_shows.get(title) {
            match movie {
                Some(movie) => return Ok(Some(movie.clone())),
                None => {
                    warn!("Did not find cover for tv show [{title}]");
                    return Ok(None);
                }
            }
        }

        let results = ctx
            .network
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

                let image_url = &format!("{}{}", TMDB_IMAGE_URL, poster).parse().unwrap();

                let cdn_file = CdnFile::from_str(&format!("tv/{}-poster-400.jpg", tv_show.id));

                let image = MediaService::image_from_url(
                    ctx,
                    image_url,
                    &cdn_file,
                    &format!("{} movie poster", tv_show.name),
                    Some(&format!("https://www.themoviedb.org/tv/{}", tv_show.id)),
                    None,
                )
                .await?;

                let tv_show = TvShow {
                    title: tv_show.name.clone(),
                    poster: image,
                    id: TvShowId::tmdb(tv_show.id),
                    link: format!("{}{}", TMDB_LINK_URL, tv_show.id).parse().unwrap(),
                };

                tv_shows.insert(tv_show.title.clone(), Some(tv_show.clone()));

                self.file.write_json(&tv_shows.clone())?;

                Ok(Some(tv_show))
            }
            None => {
                warn!("Did not find cover for tv show [{title}]");
                tv_shows.insert(title.to_string(), None);

                self.file.write_json(&tv_shows.clone())?;

                Ok(None)
            }
        }
    }
}
