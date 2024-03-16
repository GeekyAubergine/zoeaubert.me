use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::infrastructure::app_state::AppState;

mod v1;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::router())
}
