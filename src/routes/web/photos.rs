use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::error;

use crate::{
    domain::models::{
        media::{image::Image, Media},
        page::Page,
        tag::{Tag, TagSlug},
    },
    infrastructure::{app_state::AppState, repos::omni_post_repo::OmniPostRepo},
    routes::Pagination,
};

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::markdown::FormatMarkdown;
pub use crate::infrastructure::services::number::FormatNumber;

const PHOTOS_PER_PAGE: usize = 24;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosTemplate {
    page: Page,
    photos: Vec<Image>,
}

async fn index(
    State(state): State<AppState>,
    mut pagination: Query<Pagination>,
) -> Result<PhotosTemplate, (StatusCode, &'static str)> {
    pagination.set_default_pagination(PHOTOS_PER_PAGE);

    let photos: Vec<Image> = OmniPostRepo::get_posts_ordered_by_date(&state)
        .await
        .map_err(|e| {
            error!("Failed to get posts ordered by date: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get posts ordered by date",
            )
        })?
        .iter()
        .flat_map(|post| post.media())
        .filter_map(|media| match media {
            Media::Image(image) => Some(image.clone()),
            _ => None,
        })
        .clone()
        .collect();

    let total_posts_count = photos.len();

    let photos = pagination.slice(&photos);

    println!("{:?}", photos);

    let page = Page::new(
        state.site(),
        "/photos",
        Some("All Photos"),
        Some("My Photos"),
    )
    .with_pagination(pagination.page_pagination(total_posts_count, "photos"));

    Ok(PhotosTemplate { page, photos })
}
