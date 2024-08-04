use std::collections::HashMap;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use silly_names::silly_names_router;

use crate::infrastructure::app_state::AppState;
pub mod silly_names;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/silly-names", silly_names_router())
}
