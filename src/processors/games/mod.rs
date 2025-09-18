use tokio::try_join;
use tracing::info;

use crate::domain::models::games::Game;
use crate::processors::games::steam_games::process_steam_games;
use crate::{domain::models::games::Games, services::ServiceContext};

use crate::prelude::*;

pub mod steam_games;

pub async fn process_games(ctx: &ServiceContext) -> Result<Games> {
    info!("Processing Games");

    let steam_games = process_steam_games(ctx).await?;

    let mut games = Games::new();

    // for (_, game) in steam_games.games {
    //     games.add_game(Game::Steam(game));
    // }

    Ok(games)
}
