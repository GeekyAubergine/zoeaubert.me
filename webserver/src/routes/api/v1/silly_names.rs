use axum::{routing::put, Router};

use crate::{application::commands::update_silly_names_command::update_silly_names_command, infrastructure::app_state::AppState};

pub fn silly_names_router() -> Router<AppState> {
    Router::new()
        .route("/", put(update_silly_names_command))
}
