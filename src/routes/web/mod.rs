use askama::{filters::safe, Template};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::number::FormatNumber;
pub use crate::infrastructure::services::markdown::FormatMarkdown;


use crate::{
    build_data, domain::models::{blog_post::BlogPost, page::Page}, infrastructure::app_state::AppState
};

const RECENT_POSTS_COUNT: usize = 5;

pub mod blog;
pub mod hobbies;
pub mod interests;
pub mod micro_posts;
pub mod tags;
pub mod timeline;
pub mod photos;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/faq", get(faq))
        .route("/faq/", get(faq))
        .route("/now", get(now))
        .route("/now/", get(now))
        .nest("/hobbies", hobbies::router())
        .nest("/interests", interests::router())
        .nest("/blog", blog::router())
        .nest("/timeline", timeline::router())
        .nest("/tags", tags::router())
        .nest("/micros", micro_posts::router())
        .nest("/photos", photos::router())
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
    let page = Page::new(state.site(), "/", None, None);

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
    );

    let faq = state.faq_repo().get().await.text().to_owned();

    FaqTemplate { page, faq }
}

#[derive(Template)]
#[template(path = "now.html")]
pub struct NowTemplate {
    page: Page,
    now: String,
}

async fn now(State(state): State<AppState>) -> NowTemplate {
    let page = Page::new(state.site(), "/now", Some("Now"), Some("My now page"));

    let now = "testing".to_owned();

    NowTemplate { page, now }
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
