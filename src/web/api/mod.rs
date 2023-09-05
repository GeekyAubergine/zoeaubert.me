use axum::Router;

use crate::model::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
}