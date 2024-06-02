use askama::{filters::safe, Template};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;

use crate::{build_data, domain::{blog_posts::blog_post_models::BlogPost, models::page::Page}, infrastructure::app_state::AppState};
use crate::{utils::{FormatMarkdown, FormatDate}};

const RECENT_POSTS_COUNT: usize = 5;

pub mod blog;
pub mod hobbies;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/faq", get(faq))
        .nest("/hobbies", hobbies::router())
        .nest("/blog", blog::router())
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    page: Page,
    about_text: String,
    silly_names: Vec<String>,
    recent_blog_posts: Vec<BlogPost>,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(state.site(), "/", None, None, None, None, None, vec![]);

    let about_text = state.about_repo().get().await.short().to_owned();

    let silly_names = state.silly_names_repo().get().await;

    let recent_blog_posts = state
        .blog_posts_repo()
        .get_all_by_published_date()
        .await
        .iter()
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    IndexTemplate {
        page,
        silly_names,
        about_text,
        recent_blog_posts,
    }
}

#[derive(Template)]
#[template(path = "faq.html")]
pub struct FaqTemplate {
    page: Page,
    faq: String,
}

async fn faq(State(state): State<AppState>) -> FaqTemplate {
    let page = Page::new(
        state.site(),
        "/faq",
        Some("FAQ"),
        Some("Frequently Asked Questions"),
        None,
        None,
        None,
        vec![],
    );

    let faq = state.faq_repo().get().await.text().to_owned();

    FaqTemplate { page, faq }
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
