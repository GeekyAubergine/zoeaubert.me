use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::{image::Image, slug::Slug};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SteamGame {
    pub id: u32,
    pub name: String,
    pub header_image: Image,
    pub playtime: Duration,
    pub last_played: DateTime<Utc>,
    pub link_url: String,
}

impl SteamGame {
    pub fn new(
        id: u32,
        name: String,
        header_image: Image,
        playtime: Duration,
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

    pub fn slug(&self) -> Slug {
        Slug::new(&format!("/interests/games/{}/", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SteamGameAchievementUnlocked {
    pub id: String,
    pub game_id: u32,
    pub display_name: String,
    pub description: String,
    pub image: Image,
    pub unlocked_date: DateTime<Utc>,
}

impl SteamGameAchievementUnlocked {
    pub fn new(
        id: String,
        game_id: u32,
        display_name: String,
        description: String,
        image: Image,
        unlocked_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            game_id,
            display_name,
            description,
            image,
            unlocked_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SteamGameAchievementLocked {
    pub id: String,
    pub game_id: u32,
    pub display_name: String,
}

impl SteamGameAchievementLocked {
    pub fn new(id: String, game_id: u32, display_name: String) -> Self {
        Self {
            id,
            game_id,
            display_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SteamGameWithAchievements {
    pub game: SteamGame,
    pub locked_achievements: HashMap<String, SteamGameAchievementLocked>,
    pub unlocked_achievements: HashMap<String, SteamGameAchievementUnlocked>,
}

impl SteamGameWithAchievements {
    pub fn from_game(game: SteamGame) -> Self {
        Self {
            game,
            locked_achievements: HashMap::new(),
            unlocked_achievements: HashMap::new(),
        }
    }

    pub fn add_locked_achievement(&mut self, achievement: SteamGameAchievementLocked) {
        self.locked_achievements
            .insert(achievement.id.clone(), achievement);
    }

    pub fn add_unlocked_achievement(&mut self, achievement: SteamGameAchievementUnlocked) {
        self.unlocked_achievements
            .insert(achievement.id.clone(), achievement);
    }

    pub fn total_achievements(&self) -> u32 {
        (self.locked_achievements.len() + self.unlocked_achievements.len()) as u32
    }

    pub fn unlocked_achievement_count(&self) -> u32 {
        self.unlocked_achievements.len() as u32
    }

    pub fn find_all_unlocked_by_unlocked_date(&self) -> Vec<&SteamGameAchievementUnlocked> {
        let mut unlocked_achievements = self
            .unlocked_achievements
            .values()
            .collect::<Vec<&SteamGameAchievementUnlocked>>();

        unlocked_achievements.sort_by(|a, b| b.unlocked_date.cmp(&a.unlocked_date));

        unlocked_achievements
    }

    pub fn find_all_locked_by_name(&self) -> Vec<&SteamGameAchievementLocked> {
        let mut locked_achievements = self
            .locked_achievements
            .values()
            .collect::<Vec<&SteamGameAchievementLocked>>();

        locked_achievements.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        locked_achievements
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SteamGames {
    pub games: HashMap<u32, SteamGameWithAchievements>,
}

impl SteamGames {
    pub fn find_game_by_id(&self, game_id: u32) -> Option<&SteamGameWithAchievements> {
        self.games.get(&game_id)
    }

    pub fn add_game(&mut self, game: SteamGameWithAchievements) {
        self.games.insert(game.game.id, game);
    }
}
