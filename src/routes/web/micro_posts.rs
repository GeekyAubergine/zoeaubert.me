use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use crate::{
    build_data,
    domain::models::{
        media::image::Image, micro_post::MicroPost, microblog_archive::MicroblogArchivePost,
        page::Page,
    },
    infrastructure::app_state::AppState,
};

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::markdown::FormatMarkdown;
pub use crate::infrastructure::services::number::FormatNumber;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:year/:month/:day/:title", get(old_slug_redirect))
        .route("/:year/:month/:day/:title/", get(old_slug_redirect))
        .route("/:slug", get(post_page))
}

async fn old_slug_redirect(
    Path((year, month, day, title)): Path<(String, String, String, String)>,
    State(state): State<AppState>,
) -> Redirect {
    Redirect::permanent(&format!("/micros/{}-{}-{}-{}", year, month, day, title))
}

pub enum Post {
    MicroPost(MicroPost),
    MicroBlogArchive(MicroblogArchivePost),
}

impl Post {
    pub fn slug(&self) -> &str {
        match self {
            Self::MicroPost(micro_post) => micro_post.slug(),
            Self::MicroBlogArchive(archive_post) => archive_post.slug(),
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::MicroPost(micro_post) => micro_post.date(),
            Self::MicroBlogArchive(archive_post) => archive_post.date(),
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::MicroPost(micro_post) => micro_post.content(),
            Self::MicroBlogArchive(archive_post) => archive_post.content(),
        }
    }

    pub fn tags(&self) -> &Vec<crate::domain::models::tag::Tag> {
        match self {
            Self::MicroPost(micro_post) => micro_post.tags(),
            Self::MicroBlogArchive(archive_post) => archive_post.tags(),
        }
    }
}

#[derive(Template)]
#[template(path = "micro_posts/post.html")]
pub struct PostTemplate {
    page: Page,
    post: Post,
}

async fn post_page(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<PostTemplate, (StatusCode, &'static str)> {
    let micro_post = state.micro_posts_repo().get_by_slug(&id).await;

    let archive_post = state.microblog_archive_repo().get_by_slug(&id).await;

    let post = match (micro_post, archive_post) {
        (Some(micro_post), _) => Post::MicroPost(micro_post),
        (_, Some(archive_post)) => Post::MicroBlogArchive(archive_post),
        _ => return Err((StatusCode::NOT_FOUND, "Post not found")),
    };

    let page = Page::new(
        state.site(),
        &format!("/micros/{}", post.slug()),
        None,
        None,
    )
    .with_date(*post.date())
    .with_tags(post.tags().to_vec());

    Ok(PostTemplate { page, post })
}
