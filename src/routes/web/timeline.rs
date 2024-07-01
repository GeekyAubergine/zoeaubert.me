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
        media::Media,
        omni_post::OmniPost,
        page::{Page, PagePagination, PagePaginationLabel},
    },
    infrastructure::{app_state::AppState, repos::omni_post_repo::OmniPostRepo},
    routes::Pagination,
};

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::markdown::FormatMarkdown;
pub use crate::infrastructure::services::number::FormatNumber;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "timeline/index.html")]
pub struct IndexTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}
async fn index(
    State(state): State<AppState>,
    pagination: Query<Pagination>,
) -> Result<IndexTemplate, (StatusCode, &'static str)> {
    let posts = OmniPostRepo::get_posts_ordered_by_date(&state)
        .await
        .map_err(|e| {
            error!("Failed to get posts ordered by date: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get posts ordered by date",
            )
        })?;

    let total_posts_count = posts.len();

    let posts = pagination.slice(&posts);

    let page = Page::new(
        state.site(),
        "/timeline",
        Some("Timeline"),
        Some("My timeline"),
    )
    .with_pagination(pagination.page_pagination(total_posts_count, "timeline"));

    Ok(IndexTemplate { page, posts })
}
