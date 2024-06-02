use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

use crate::utils::{FormatDate, FormatNumber};

pub mod games;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .nest("/games", games::router())
}

#[derive(Template)]
#[template(path = "interests/index.html")]
pub struct IndexTemplate {
    page: Page,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/interests/",
        Some("Interests"),
        Some("My Interests"),
        None,
        None,
        None,
        vec![],
    );

    IndexTemplate { page }
}
