use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::error;
use uuid::Uuid;

use crate::{
    domain::models::{
        media::Media,
        omni_post::OmniPost,
        page::{Page, PagePagination, PagePaginationLabel},
        tag::Tag, UuidIdentifiable,
    },
    infrastructure::{
        app_state::AppState,
        query_services::{
            omni_post_query_service::{
                find_omni_posts_by_date, OmniPostFilterFlags, OmniPostQueryService,
            },
            tags_query_service::find_tags_for_entities,
        },
    },
    routes::Pagination,
    ResponseResult,
};

pub use crate::infrastructure::formatters::format_date::FormatDate;
pub use crate::infrastructure::formatters::format_markdown::FormatMarkdown;
pub use crate::infrastructure::formatters::format_number::FormatNumber;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}

pub struct Post {
    pub post: OmniPost,
    pub media: Vec<Media>,
    pub tags: Vec<Tag>,
}

impl Post {
    pub fn key(&self) -> String {
        self.post.key()
    }

    pub fn permalink(&self) -> String{
        self.post.permalink()
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.post.date()
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }
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
) -> ResponseResult<IndexTemplate> {
    let posts = find_omni_posts_by_date(
        &state,
        OmniPostFilterFlags::filter_non_album_photo_and_game_achievement(),
    )
    .await?;

    let tags = find_tags_for_entities(&posts, state.tags_repo()).await?;

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
