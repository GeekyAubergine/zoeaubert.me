use chrono::{DateTime, Utc};

use crate::{
    domain::{models::games::Game, repositories::GamesRepo, state::State},
    prelude::*,
};

pub async fn find_game_by_id(state: &impl State, id: u32) -> Result<Option<Game>> {
    state.games_repo().find_by_game_id(id).await
}

pub async fn find_all_games(state: &impl State) -> Result<Vec<Game>> {
    let mut data = state.games_repo().find_all_games().await?;

    Ok(data)
}

pub async fn find_all_games_by_most_played(state: &impl State) -> Result<Vec<Game>> {
    let mut data = state.games_repo().find_all_games().await?;

    data.sort_by(|a, b| b.playtime.cmp(&a.playtime));

    Ok(data)
}

pub async fn find_games_last_updated_at(state: &impl State) -> Result<Option<DateTime<Utc>>> {
    state.games_repo().find_most_recently_updated_at().await
}

pub async fn commit_game(state: &impl State, game: &Game) -> Result<()> {
    state.games_repo().commit(game).await
}
