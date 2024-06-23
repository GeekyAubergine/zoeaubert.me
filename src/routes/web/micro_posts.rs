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
    domain::{
        blog_posts::blog_post_models::BlogPost,
        micro_posts::micro_posts_models::MicroPost,
        models::{media::image::Image, page::Page},
    },
    infrastructure::app_state::AppState,
};

use crate::utils::{FormatDate, FormatMarkdown, FormatNumber};

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

#[derive(Template)]
#[template(path = "micro_posts/post.html")]
pub struct PostTemplate {
    page: Page,
    post: MicroPost,
}

async fn post_page(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<PostTemplate, (StatusCode, &'static str)> {
    let post = state
        .micro_posts_repo()
        .get_by_slug(&id)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))?;

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
