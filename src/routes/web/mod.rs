use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
    charlie: &'a str,
    build_date: &'a str,
    page: Page,
}

async fn index(State(state): State<AppState>) -> HelloTemplate<'static> {
    let page = Page::new(state.site(), "/", Some("Home"), None, None);

    HelloTemplate {
        name: "world",
        charlie: "Charlie",
        build_date: build_data::BUILD_DATE,
        page,
    }
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
