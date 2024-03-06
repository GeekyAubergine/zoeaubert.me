use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::protobuf::Protobuf;
use serde::Serialize;

use crate::{domain::models::about::About, infrastructure::app_state::AppState, prelude::*};

#[derive(Debug, Clone, Serialize)]
pub struct ResponseAboutData {
    about_text: String,
}

impl From<About> for ResponseAboutData {
    fn from(about: About) -> Self {
        Self {
            about_text: about.text().to_string(),
        }
    }
}

pub async fn query_get_about_data(State(state): State<AppState>) -> Json<ResponseAboutData> {
    let about = state.about_repo().get_about().await;

    Json(about.into())
}
