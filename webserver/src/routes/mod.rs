use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models::page::{PagePagination, PagePaginationLabel},
    infrastructure::app_state::AppState,
};

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

const PER_PAGE: usize = 25;

#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Pagination {
    fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    fn set_default_pagination(&mut self, default_per_page: usize) {
        if self.per_page.is_none() {
            self.per_page = Some(default_per_page);
        }
    }

    fn per_page(&self) -> usize {
        self.per_page.unwrap_or(PER_PAGE)
    }

    fn slice<T: Clone>(&self, entities: &[T]) -> Vec<T> {
        entities
            .iter()
            .skip((self.page() - 1) * self.per_page())
            .take(self.per_page())
            .cloned()
            .collect::<Vec<T>>()
    }

    fn page_pagination(&self, total_entities_count: usize, slug: &str) -> PagePagination {
        let previous_nav = match total_entities_count > self.page() * self.per_page() {
            true => Some(PagePaginationLabel::new(
                &format!("{}?page={}", slug, self.page() + 1),
                "Older posts",
            )),
            false => None,
        };

        let next_nav = match self.page() {
            1 => None,
            2 => Some(PagePaginationLabel::new(
                &format!("{}", slug),
                "Newer posts",
            )),
            _ => Some(PagePaginationLabel::new(
                &format!("{}?page={}", slug, self.page() - 1),
                "Newer posts",
            )),
        };

        PagePagination::new(previous_nav, next_nav)
    }
}
