use std::collections::HashMap;

use axum::{extract::State, Json};
use axum_extra::protobuf::Protobuf;
use serde::Serialize;

use crate::{
    domain::models::lego::{LegoMinifig, LegoSet},
    infrastructure::app_state::AppState,
    prelude::*,
};

#[derive(Debug, Clone, Serialize)]
struct ResponseLegoSet {
    id: u32,
    name: String,
    number: String,
    category: String,
    pieces: u32,
    image_url: String,
    thumbnail_url: String,
    link_url: String,
    quantity: u32,
}

impl From<LegoSet> for ResponseLegoSet {
    fn from(set: LegoSet) -> Self {
        Self {
            id: set.key(),
            name: set.name().to_string(),
            number: set.number().to_string(),
            category: set.category().to_string(),
            pieces: set.pieces(),
            image_url: set.image().to_string(),
            thumbnail_url: set.thumbnail().to_string(),
            link_url: set.link().to_string(),
            quantity: set.quantity(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ResponseLegoMinifig {
    id: String,
    name: String,
    category: String,
    owned_in_sets: u32,
    owned_loose: u32,
    image_url: String,
}

impl From<LegoMinifig> for ResponseLegoMinifig {
    fn from(minifig: LegoMinifig) -> Self {
        Self {
            id: minifig.key().to_owned(),
            name: minifig.name().to_string(),
            category: minifig.category().to_string(),
            owned_in_sets: minifig.owned_in_sets(),
            owned_loose: minifig.owned_loose(),
            image_url: minifig.image_url().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseLegoData {
    sets: HashMap<u32, ResponseLegoSet>,
    set_ids: Vec<u32>,
    minifigs: HashMap<String, ResponseLegoMinifig>,
    minifig_ids: Vec<String>,
    total_pieces: u32,
}

pub async fn get_all_lego_data_query(State(state): State<AppState>) -> Json<ResponseLegoData> {
    let sets = state
        .lego_repo()
        .get_all_sets()
        .await
        .into_iter()
        .map(|(key, set)| (key, set.into()))
        .collect::<HashMap<u32, ResponseLegoSet>>();

    let set_ids = state.lego_repo().get_most_piece_sets().await;

    let minifigs = state
        .lego_repo()
        .get_all_minifigs()
        .await
        .into_iter()
        .map(|(key, minifig)| (key, minifig.into()))
        .collect::<HashMap<String, ResponseLegoMinifig>>();
    let minifig_ids = state.lego_repo().get_most_owned_minifigs().await;

    let total_pieces = state.lego_repo().get_total_pieces().await;

    Json(ResponseLegoData {
        sets,
        set_ids,
        minifigs,
        minifig_ids,
        total_pieces,
    })
}
