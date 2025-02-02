use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{image::Image, slug::Slug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGame {
    pub id: u32,
    pub name: String,
    pub header_image: Image,
    pub playtime: u32,
    pub last_played: DateTime<Utc>,
    pub link_url: String,
}

impl SteamGame {
    pub fn new(
        id: u32,
        name: String,
        header_image: Image,
        playtime: u32,
        last_played: DateTime<Utc>,
        link_url: String,
    ) -> Self {
        Self {
            id,
            name,
            header_image,
            playtime,
            last_played,
            link_url,
        }
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime as f32 / 60.0
    }

    pub fn slug(&self) -> Slug {
        Slug::new(&format!("/interests/games/{}/", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameAchievementUnlocked {
    pub id: String,
    pub game_id: u32,
    pub display_name: String,
    pub description: String,
    pub image: Image,
    pub unlocked_date: DateTime<Utc>,
    pub global_unlocked_percentage: f32,
}

impl SteamGameAchievementUnlocked {
    pub fn new(
        id: String,
        game_id: u32,
        display_name: String,
        description: String,
        image: Image,
        unlocked_date: DateTime<Utc>,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            game_id,
            display_name,
            description,
            image,
            unlocked_date,
            global_unlocked_percentage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameAchievementLocked {
    pub id: String,
    pub game_id: u32,
    pub display_name: String,
    pub description: String,
    pub image: Image,
    pub global_unlocked_percentage: f32,
}

impl SteamGameAchievementLocked {
    pub fn new(
        id: String,
        game_id: u32,
        display_name: String,
        description: String,
        image: Image,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            game_id,
            display_name,
            description,
            image,
            global_unlocked_percentage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SteamGameAchievement {
    Unlocked(SteamGameAchievementUnlocked),
    Locked(SteamGameAchievementLocked),
}

impl SteamGameAchievement {
    pub fn id(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => &achievement.id,
            Self::Locked(achievement) => &achievement.id,
        }
    }

    pub fn game_id(&self) -> u32 {
        match self {
            Self::Unlocked(achievement) => achievement.game_id,
            Self::Locked(achievement) => achievement.game_id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => &achievement.display_name,
            Self::Locked(achievement) => &achievement.display_name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => &achievement.description,
            Self::Locked(achievement) => &achievement.description,
        }
    }

    pub fn is_unlocked(&self) -> bool {
        matches!(self, Self::Unlocked(_))
    }
}
