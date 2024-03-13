use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

pub mod games;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .nest("/games", games::router())
}

#[derive(Template)]
#[template(path = "hobbies/index.html")]
pub struct IndexTemplate {
    page: Page,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/hobbies/",
        Some("Hobbies"),
        Some("My Hobbies"),
        None,
        None,
        None,
        vec![],
    );

    IndexTemplate { page }
}
