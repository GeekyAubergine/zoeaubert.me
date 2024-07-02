use askama::Template;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::de;
use tracing::error;

use crate::domain::models::{media::image::Image, page::Page};

use crate::infrastructure::app_state::AppState;
use crate::infrastructure::services::album::{cover_photos_for_album, ordered_photos_for_album};
use crate::infrastructure::services::date::FormatDate;
use crate::infrastructure::services::markdown::FormatMarkdown;
use crate::infrastructure::services::number::FormatNumber;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(albums_list))
        .route("/all", get(all_albums_photos))
}

struct AlbumListItem {
    title: String,
    date: DateTime<Utc>,
    images: Vec<Image>,
    cover_images: Vec<Image>,
    permalink: String,
}

impl AlbumListItem {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn images(&self) -> &[Image] {
        &self.images
    }

    pub fn cover_images(&self) -> &[Image] {
        &self.cover_images
    }

    pub fn permalink(&self) -> &str {
        &self.permalink
    }
}

struct AlbumYearGroup {
    year: String,
    albums: Vec<AlbumListItem>,
}

impl AlbumYearGroup {
    pub fn year(&self) -> &str {
        &self.year
    }

    pub fn albums(&self) -> &[AlbumListItem] {
        &self.albums
    }
}

#[derive(Template)]
#[template(path = "albums/index.html")]
pub struct AlbumsListTemplate {
    page: Page,
    albums_by_year: Vec<AlbumYearGroup>,
}

async fn albums_list(
    State(state): State<AppState>,
) -> Result<AlbumsListTemplate, (StatusCode, &'static str)> {
    let page = Page::new(state.site(), "/albums", Some("Albums"), None);

    let albums = state.albums_repo().group_by_year().await;

    let albums_by_year = albums
        .iter()
        .map(|(year, albums)| {
            let albums = albums
                .iter()
                .map(|album| AlbumListItem {
                    title: album.title().to_owned(),
                    date: album.date().clone(),
                    images: ordered_photos_for_album(album)
                        .iter()
                        .map(|p| p.small_image().clone())
                        .collect(),
                    cover_images: cover_photos_for_album(album)
                        .iter()
                        .map(|p| p.small_image().clone())
                        .collect(),
                    permalink: album.permalink(),
                })
                .collect();

            AlbumYearGroup {
                year: year.to_string(),
                albums,
            }
        })
        .collect();

    Ok(AlbumsListTemplate {
        page,
        albums_by_year,
    })
}

#[derive(Template)]
#[template(path = "albums/all_photos.html")]
pub struct AllAlbumsPhotosTemplate {
    page: Page,
    albums: Vec<AlbumListItem>,
}

async fn all_albums_photos(
    State(state): State<AppState>,
) -> Result<AllAlbumsPhotosTemplate, (StatusCode, &'static str)> {
    let page = Page::new(state.site(), "/albums", Some("Albums"), None);

    let albums = state.albums_repo().get_all_by_date().await;

    let albums = albums
        .iter()
        .map(|album| AlbumListItem {
            title: album.title().to_owned(),
            date: album.date().clone(),
            images: ordered_photos_for_album(album)
                .iter()
                .map(|p| p.small_image().clone())
                .collect(),
            cover_images: cover_photos_for_album(album)
                .iter()
                .map(|p| p.small_image().clone())
                .collect(),
            permalink: album.permalink(),
        })
        .collect();

    Ok(AllAlbumsPhotosTemplate { page, albums })
}
