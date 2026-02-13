use tracing::info;

use crate::domain::models::games::Game;
use crate::processors::games::processor_steam_games::load_steam_games;
use crate::{domain::models::games::Games, services::ServiceContext};

use crate::prelude::*;

pub mod processor_steam_games;

pub fn load_games(ctx: &ServiceContext) -> Result<Games> {
    info!("Processing Games");

    let steam_games = load_steam_games(ctx)?;

    let mut games = Games::new();

    for (_, game) in steam_games.games {
        games.add_game(Game::Steam(game));
    }

    Ok(games)
}
