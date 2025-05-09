use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use steam::SteamGameWithAcheivements;

use crate::{
    domain::{
        repositories::{SteamAchievementsRepo, SteamGamesRepo},
        state::State,
    },
    prelude::Result,
};

use super::image::Image;

pub mod steam;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum GameId {
    Steam(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Game {
    Steam(SteamGameWithAcheivements),
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
}

pub struct Games {
    pub games: HashMap<GameId, Game>,
}

impl Games {
    pub async fn from_state(state: &impl State) -> Result<Self> {
        let mut games = Self {
            games: HashMap::new(),
        };

        for game in state.steam_games_repo().find_all().await? {
            let game = steam::SteamGame {
                id: game.id,
                name: game.name,
                header_image: game.header_image,
                playtime: Duration::from_secs(game.playtime as u64 * 60),
                last_played: game.last_played,
                link_url: game.link_url,
            };

            let mut game = SteamGameWithAcheivements::from_game(game);

            for achievement in state
                .steam_achievements_repo()
                .find_all_locked_by_name(game.game.id)
                .await?
                .iter()
            {
                game.add_locked_achievement(steam::SteamGameAchievementLocked {
                    id: achievement.id.clone(),
                    game_id: achievement.game_id,
                    display_name: achievement.display_name.clone(),
                    description: achievement.description.clone(),
                    image: achievement.image.clone(),
                    global_unlocked_percentage: achievement.global_unlocked_percentage,
                });
            }

            for achievement in state
                .steam_achievements_repo()
                .find_all_unlocked_by_unlocked_date(game.game.id)
                .await?
                .iter()
            {
                game.add_unlocked_achievement(steam::SteamGameAchievementUnlocked {
                    id: achievement.id.clone(),
                    game_id: achievement.game_id,
                    display_name: achievement.display_name.clone(),
                    description: achievement.description.clone(),
                    image: achievement.image.clone(),
                    unlocked_date: achievement.unlocked_date,
                    global_unlocked_percentage: achievement.global_unlocked_percentage,
                });
            }
        }

        Ok(games)
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
