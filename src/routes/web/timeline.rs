use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::error;

use crate::{
    domain::{
        models::page::{Page, PagePagination, PagePaginationLabel},
        omni_post::{omni_post_models::OmniPost, omni_post_repo::OmniPostRepo},
    },
    infrastructure::app_state::AppState, routes::Pagination,
};

use crate::utils::{FormatDate, FormatMarkdown, FormatNumber};

const POSTS_PER_PAGE: usize = 25;

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

    let posts = posts
        .iter()
        .skip((pagination.page() - 1) * pagination.per_page())
        .take(POSTS_PER_PAGE)
        .cloned()
        .collect::<Vec<OmniPost>>();

    let previous_nav = match total_posts_count > pagination.page() * pagination.per_page() {
        true => Some(PagePaginationLabel::new(
            &format!("/timeline?page={}", pagination.page() + 1),
            "Older posts",
        )),
        false => None,
    };

    let next_nav = match pagination.page() {
        1 => None,
        _ => Some(PagePaginationLabel::new(
            &format!("/timeline?page={}", pagination.page() - 1),
            "Newer posts",
        )),
    };

    let page = Page::new(
        state.site(),
        "/timeline",
        Some("Timeline"),
        Some("My timeline"),
        None,
        None,
        None,
        vec![],
    )
    .set_no_index()
    .with_pagination(PagePagination::new(previous_nav, next_nav));

    Ok(IndexTemplate { page, posts })
}
