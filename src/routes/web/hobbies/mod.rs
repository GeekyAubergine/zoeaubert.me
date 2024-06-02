use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use crate::{build_data, domain::models::page::Page, infrastructure::app_state::AppState};

use crate::utils::{FormatDate, FormatNumber};

pub fn router() -> Router<AppState> {
    Router::new().route("/*path", get(redirect_to_interests))
}

async fn redirect_to_interests(Path(path): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/interests/{}", path))
}
