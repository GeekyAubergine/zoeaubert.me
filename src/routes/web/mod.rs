use askama::{filters::safe, Template};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::utils::FormatMarkdown;
use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

pub mod hobbies;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/faq", get(faq))
        .nest("/hobbies", hobbies::router())
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    page: Page,
    about_text: String,
    silly_names: Vec<String>,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/",
        Some("Home"),
        None,
        None,
        None,
        None,
        vec![],
    );

    let about_text = state.about_repo().get().await.short().to_owned();

    let silly_names = state.silly_names_repo().get().await;

    IndexTemplate {
        page,
        silly_names,
        about_text,
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
