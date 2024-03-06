use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    domain::models::{about::About, faq::Faq},
    infrastructure::app_state::AppState,
    prelude::*,
};

#[derive(Debug, Clone, Serialize)]
pub struct ResponseFaqData {
    faq_text: String,
}

impl From<Faq> for ResponseFaqData {
    fn from(faq: Faq) -> Self {
        Self {
            faq_text: faq.text().to_string(),
        }
    }
}

pub async fn get_faq_data_query(State(state): State<AppState>) -> Json<ResponseFaqData> {
    let faq = state.faq_repo().get_faq().await;

    Json(faq.into())
}
