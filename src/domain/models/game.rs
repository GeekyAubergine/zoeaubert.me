use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        repositories::{SteamAchievementsRepo, SteamGamesRepo},
        state::State,
    },
    prelude::Result,
};

use super::{
    image::Image,
    league::LeagueGameStats,
    steam::{SteamGame, SteamGameAchievementLocked, SteamGameAchievementUnlocked},
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum GameId {
    Steam(u32),
}

#[derive(Debug, Clone)]
pub enum Game {
    Steam {
        game: SteamGame,
        achievements_locked: HashMap<GameAchievementId, SteamGameAchievementLocked>,
        achievements_unlocked: HashMap<GameAchievementId, SteamGameAchievementUnlocked>,
    },
}

impl Game {
    pub fn id(&self) -> GameId {
        match self {
            Game::Steam { game, .. } => GameId::Steam(game.id),
        }
    }

    pub fn slug_partial(&self) -> String {
        match self {
            Game::Steam { game, .. } => game.id.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Game::Steam { game, .. } => &game.name,
        }
    }

    pub fn playtime(&self) -> Duration {
        match self {
            Game::Steam { game, .. } => Duration::from_secs((game.playtime * 60) as u64),
        }
    }

    pub fn image(&self) -> &Image {
        match self {
            Game::Steam { game, .. } => &game.header_image,
        }
    }

    pub fn last_played(&self) -> &DateTime<Utc> {
        match self {
            Game::Steam { game, .. } => &game.last_played,
        }
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime().as_secs_f32() / 3600.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum GameAchievementId {
    Steam(String),
}

#[derive(Debug, Clone)]
pub struct GameAchievementUnlocked {
    id: GameAchievementId,
    game_id: u32,
    name: String,
    description: String,
    image: Image,
    unlocked_at: DateTime<Utc>,
    global_unlocked_percentage: f32,
}

#[derive(Debug, Clone)]
pub struct GameAchievementLocked {
    id: GameAchievementId,
    game_id: u32,
    name: String,
    description: String,
    image: Image,
    global_unlocked_percentage: f32,
}

#[derive(Debug, Clone)]
pub enum GameAchievement {
    Unlocked(GameAchievementUnlocked),
    Locked(GameAchievementLocked),
}

impl GameAchievement {
    pub fn name(&self) -> &str {
        match self {
            GameAchievement::Unlocked(ach) => &ach.name,
            GameAchievement::Locked(ach) => &ach.name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            GameAchievement::Unlocked(ach) => &ach.description,
            GameAchievement::Locked(ach) => &ach.description,
        }
    }

    pub fn image(&self) -> &Image {
        match self {
            GameAchievement::Unlocked(ach) => &ach.image,
            GameAchievement::Locked(ach) => &ach.image,
        }
    }

    pub fn global_unlocked_percentage(&self) -> f32 {
        match self {
            GameAchievement::Unlocked(ach) => ach.global_unlocked_percentage,
            GameAchievement::Locked(ach) => ach.global_unlocked_percentage,
        }
    }

    pub fn is_unlocked(&self) -> bool {
        match self {
            GameAchievement::Unlocked(_) => true,
            GameAchievement::Locked(_) => false,
        }
    }
}

impl From<SteamGameAchievementUnlocked> for GameAchievementUnlocked {
    fn from(achievment: SteamGameAchievementUnlocked) -> Self {
        GameAchievementUnlocked {
            id: GameAchievementId::Steam(achievment.id),
            game_id: achievment.game_id,
            name: achievment.display_name,
            description: achievment.description,
            image: achievment.image,
            unlocked_at: achievment.unlocked_date,
            global_unlocked_percentage: achievment.global_unlocked_percentage,
        }
    }
}

impl From<SteamGameAchievementLocked> for GameAchievementLocked {
    fn from(achievment: SteamGameAchievementLocked) -> Self {
        GameAchievementLocked {
            id: GameAchievementId::Steam(achievment.id),
            game_id: achievment.game_id,
            name: achievment.display_name,
            description: achievment.description,
            image: achievment.image,
            global_unlocked_percentage: achievment.global_unlocked_percentage,
        }
    }
}

impl From<SteamGameAchievementUnlocked> for GameAchievement {
    fn from(achievment: SteamGameAchievementUnlocked) -> Self {
        GameAchievement::Unlocked(GameAchievementUnlocked::from(achievment))
    }
}

impl From<SteamGameAchievementLocked> for GameAchievement {
    fn from(achievment: SteamGameAchievementLocked) -> Self {
        GameAchievement::Locked(GameAchievementLocked::from(achievment))
    }
}

pub struct GameWithAchievements {
    pub game: Game,
    pub locked_achievements: HashMap<GameAchievementId, GameAchievementLocked>,
    pub unlocked_achievements: HashMap<GameAchievementId, GameAchievementUnlocked>,
}

impl GameWithAchievements {
    pub fn total_achievements(&self) -> u32 {
        (self.locked_achievements.len() + self.unlocked_achievements.len()) as u32
    }

    pub fn unlocked_achievevment_count(&self) -> u32 {
        self.unlocked_achievements.len() as u32
    }

    pub fn achievement_unlocked_percentage(&self) -> f32 {
        let unlocked_count = self.unlocked_achievements.len() as f32;
        let total_count =
            (self.unlocked_achievements.len() + self.locked_achievements.len()) as f32;

        unlocked_count / total_count.max(1.0)
    }
}
