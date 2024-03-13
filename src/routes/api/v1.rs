use std::collections::HashMap;

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::{
    application::queries::{
        // games::get_all_games_data_query::get_all_games_data_query,
        lego::get_all_lego_data_query::get_all_lego_data_query, about::get_about_data_query::query_get_about_data, faq::get_faq_data_query::get_faq_data_query,
    },
    infrastructure::app_state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/lego", get(get_all_lego_data_query))
        // .route("/games", get(get_all_games_data_query))
        .route("/about", get(query_get_about_data))
        .route("/faq", get(get_faq_data_query))
}
