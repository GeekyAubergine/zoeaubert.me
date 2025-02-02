use std::time::Duration;

use chrono::{DateTime, Utc};

use super::{
    image::Image,
    league::LeagueGameStats,
    steam::{SteamGame, SteamGameAchievementLocked, SteamGameAchievementUnlocked},
};

#[derive(Debug, Clone)]
pub enum Game {
    Steam(SteamGame),
    League(LeagueGameStats),
}

impl Game {
    pub fn slug_partial(&self) -> String {
        match self {
            Game::Steam(game) => game.id.to_string(),
            Game::League(game) => "league".to_string(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Game::Steam(game) => &game.name,
            Game::League(_) => &"League of Legends",
        }
    }

    pub fn playtime(&self) -> Duration {
        match self {
            Game::Steam(game) => Duration::from_secs((game.playtime * 60) as u64),
            Game::League(game) => game.playtime,
        }
    }

    pub fn image(&self) -> &Image {
        match self {
            Game::Steam(game) => &game.header_image,
            Game::League(_) => unimplemented!(),
        }
    }

    pub fn last_played(&self) -> &DateTime<Utc> {
        match self {
            Game::Steam(game) => &game.last_played,
            Game::League(game) => &game.last_played,
        }
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime().as_secs_f32() / 3600.0
    }
}

impl From<SteamGame> for Game {
    fn from(game: SteamGame) -> Self {
        Game::Steam(game)
    }
}

impl From<LeagueGameStats> for Game {
    fn from(game: LeagueGameStats) -> Self {
        Game::League(game)
    }
}

#[derive(Debug, Clone)]
pub struct GameAchievmentUnlocked {
    id: String,
    game_id: u32,
    name: String,
    description: String,
    image: Image,
    unlocked_at: DateTime<Utc>,
    global_unlocked_percentage: f32,
}

#[derive(Debug, Clone)]
pub struct GameAchievmentLocked {
    id: String,
    game_id: u32,
    name: String,
    description: String,
    image: Image,
    global_unlocked_percentage: f32,
}

#[derive(Debug, Clone)]
pub enum GameAchievement {
    Unlocked(GameAchievmentUnlocked),
    Locked(GameAchievmentLocked),
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

impl From<SteamGameAchievementUnlocked> for GameAchievmentUnlocked {
    fn from(achievment: SteamGameAchievementUnlocked) -> Self {
        GameAchievmentUnlocked {
            id: achievment.id,
            game_id: achievment.game_id,
            name: achievment.display_name,
            description: achievment.description,
            image: achievment.image,
            unlocked_at: achievment.unlocked_date,
            global_unlocked_percentage: achievment.global_unlocked_percentage,
        }
    }
}

impl From<SteamGameAchievementLocked> for GameAchievmentLocked {
    fn from(achievment: SteamGameAchievementLocked) -> Self {
        GameAchievmentLocked {
            id: achievment.id,
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
        GameAchievement::Unlocked(GameAchievmentUnlocked::from(achievment))
    }
}

impl From<SteamGameAchievementLocked> for GameAchievement {
    fn from(achievment: SteamGameAchievementLocked) -> Self {
        GameAchievement::Locked(GameAchievmentLocked::from(achievment))
    }
}
