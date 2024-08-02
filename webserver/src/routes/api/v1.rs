use std::collections::HashMap;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::infrastructure::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}
