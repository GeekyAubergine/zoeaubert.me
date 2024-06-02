use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::infrastructure::app_state::AppState;

mod api;
mod web;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::router())
        .nest("/", web::router())
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Pagination {
    fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    fn per_page(&self) -> usize {
        self.per_page.unwrap_or(25)
    }
}
