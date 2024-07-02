use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{
    build_data,
    domain::models::{blog_post::BlogPost, media::image::Image, page::Page},
    infrastructure::app_state::AppState,
};

use crate::infrastructure::services::date::FormatDate;
use crate::infrastructure::services::number::FormatNumber;
use crate::infrastructure::services::markdown::FormatMarkdown;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/:id", get(post_page))
        .route("/:id/", get(post_page))
}

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct IndexTemplate {
    page: Page,
    blog_posts: Vec<BlogPost>,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/blog",
        Some("Blog Posts"),
        Some("My blog posts"),
    );

    let blog_posts = state.blog_posts_repo().get_all_by_published_date().await;

    IndexTemplate { page, blog_posts }
}

#[derive(Template)]
#[template(path = "blog/post.html")]
pub struct PostTemplate {
    page: Page,
    post: BlogPost,
}

async fn post_page(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<PostTemplate, (StatusCode, &'static str)> {
    let post = state
        .blog_posts_repo()
        .get_by_slug(&id)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))?;

    let page = Page::new(
        state.site(),
        &format!("/blog/{}", post.slug()),
        Some(post.title()),
        Some(post.description()),
    )
    .with_date(*post.date())
    .with_tags(post.tags().to_vec());

    let page = match post.hero_image() {
        Some(image) => page.with_image(image.clone()),
        None => page,
    };

    Ok(PostTemplate { page, post })
}
