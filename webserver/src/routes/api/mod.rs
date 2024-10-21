use std::sync::Arc;

use axum::{http::StatusCode, middleware, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::infrastructure::{app_state::AppState, services::auth_service::auth_middleware};

mod v1;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::router())
        .layer(middleware::from_fn(auth_middleware))
}
