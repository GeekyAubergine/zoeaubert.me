use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use steam::SteamGameWithAchievements;

use crate::prelude::Result;

use super::image::Image;

pub mod steam;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum GameId {
    Steam(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Game {
    Steam(SteamGameWithAchievements),
}

impl Game {
    pub fn id(&self) -> GameId {
        match self {
            Game::Steam(game) => GameId::Steam(game.game.id),
        }
    }

    pub fn last_played(&self) -> &DateTime<Utc> {
        match self {
            Game::Steam(game) => &game.game.last_played,
        }
    }

    pub fn playtime(&self) -> &Duration {
        match self {
            Game::Steam(game) => &game.game.playtime,
        }
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime().as_secs() as f32 / (60.0 * 60.0)
    }

    pub fn achievement_unlocked_percentage(&self) -> f32 {
        match self {
            Game::Steam(game) => game.achievement_unlocked_percentage(),
        }
    }

    pub fn image(&self) -> &Image {
        match self {
            Game::Steam(game) => &game.game.header_image,
        }
    }
}

pub struct Games {
    pub games: HashMap<GameId, Game>,
}

impl Games {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn find_by_id(&self, game_id: &GameId) -> Option<&Game> {
        self.games.get(game_id)
    }

    pub fn find_by_most_recently_played(&self) -> Vec<&Game> {
        let mut games = self.games.values().collect::<Vec<&Game>>();

        games.sort_by(|a, b| b.last_played().cmp(&a.last_played()));

        games
    }

    pub fn find_by_most_most_played(&self) -> Vec<&Game> {
        let mut games = self.games.values().collect::<Vec<&Game>>();

        games.sort_by(|a, b| b.playtime().cmp(&a.playtime()));

        games
    }

    pub fn find_by_most_highest_achievement_unlocked_percentage(&self) -> Vec<&Game> {
        let mut games = self.games.values().collect::<Vec<&Game>>();

        games.sort_by(|a, b| {
            let a_percentage = a.achievement_unlocked_percentage();
            let b_percentage = b.achievement_unlocked_percentage();

            b_percentage.partial_cmp(&a_percentage).unwrap()
        });

        games
    }

    pub fn find_all(&self) -> Vec<&Game> {
        self.games.values().collect()
    }

    pub fn find_total_playtime(&self) -> Duration {
        self.games.values().map(|g| g.playtime()).sum()
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.insert(game.id(), game);
    }
}
