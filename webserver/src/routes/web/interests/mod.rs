use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

pub use crate::infrastructure::formatters::format_date::FormatDate;
pub use crate::infrastructure::formatters::format_markdown::FormatMarkdown;
pub use crate::infrastructure::formatters::format_number::FormatNumber;

pub mod games;
pub mod lego;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .nest("/games", games::router())
        .nest("/lego", lego::router())
}

#[derive(Template)]
#[template(path = "interests/interests_list.html")]
pub struct IndexTemplate {
    page: Page,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/interests",
        Some("Interests"),
        Some("My Interests"),
    );

    IndexTemplate { page }
}
