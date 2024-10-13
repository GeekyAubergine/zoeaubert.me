use chrono::{DateTime, Utc};

use crate::{
    domain::{
        models::lego::{LegoMinifig, LegoSet},
        repositories::LegoRepo,
        state::State,
    },
    prelude::*,
};

pub async fn find_all_lego_sets(state: &impl State) -> Result<Vec<LegoSet>> {
    state.lego_repo().find_all_sets().await
}

pub async fn find_all_lego_minifiigs(state: &impl State) -> Result<Vec<LegoMinifig>> {
    state.lego_repo().find_all_minifigs().await
}

pub async fn find_total_lego_pieces(state: &impl State) -> Result<u32> {
    state.lego_repo().find_total_pieces().await
}

pub async fn find_total_lego_sets(state: &impl State) -> Result<u32> {
    state.lego_repo().find_total_sets().await
}

pub async fn find_total_lego_minifiigs(state: &impl State) -> Result<u32> {
    state.lego_repo().find_total_minifigs().await
}

pub async fn find_lego_last_updated_at(state: &impl State) -> Result<Option<DateTime<Utc>>> {
    state.lego_repo().find_last_updated_at().await
}

pub async fn commit_lego_set(state: &impl State, set: &LegoSet) -> Result<()> {
    state.lego_repo().commit_set(set).await
}

pub async fn commit_lego_minifig(state: &impl State, minifig: &LegoMinifig) -> Result<()> {
    state.lego_repo().commit_minifig(minifig).await
}
