use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GameAchievementUnlocked {
    id: String,
    game_id: u32,
    display_name: String,
    description: String,
    image_url: String,
    unlocked_date: DateTime<Utc>,
    global_unlocked_percentage: f32,
}

impl GameAchievementUnlocked {
    pub fn new(
        id: String,
        game_id: u32,
        display_name: String,
        description: String,
        image_url: String,
        unlocked_date: DateTime<Utc>,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            game_id,
            display_name,
            description,
            image_url,
            unlocked_date,
            global_unlocked_percentage,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn game_id(&self) -> u32 {
        self.game_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub fn unlocked_date(&self) -> &DateTime<Utc> {
        &self.unlocked_date
    }

    pub fn global_unlocked_percentage(&self) -> f32 {
        self.global_unlocked_percentage
    }
}

#[derive(Debug, Clone)]
pub struct GameAchievementLocked {
    id: String,
    game_id: u32,
    display_name: String,
    description: String,
    image_url: String,
    global_unlocked_percentage: f32,
}

impl GameAchievementLocked {
    pub fn new(
        id: String,
        game_id: u32,
        display_name: String,
        description: String,
        image_url: String,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            game_id,
            display_name,
            description,
            image_url,
            global_unlocked_percentage,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn game_id(&self) -> u32 {
        self.game_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub fn global_unlocked_percentage(&self) -> f32 {
        self.global_unlocked_percentage
    }
}

#[derive(Debug, Clone)]
pub enum GameAchievement {
    Unlocked(GameAchievementUnlocked),
    Locked(GameAchievementLocked),
}

impl GameAchievement {
    pub fn id(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => achievement.id(),
            Self::Locked(achievement) => achievement.id(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => achievement.display_name(),
            Self::Locked(achievement) => achievement.display_name(),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Unlocked(achievement) => achievement.description(),
            Self::Locked(achievement) => achievement.description(),
        }
    }

    pub fn is_unlocked(&self) -> bool {
        matches!(self, Self::Unlocked(_))
    }
}

#[derive(Debug, Clone)]
pub struct GameAchievements {
    game_id: u32,
    achievements: HashMap<String, GameAchievement>,
}

impl GameAchievements {
    pub fn new(game_id: u32) -> Self {
        Self {
            game_id,
            achievements: HashMap::new(),
        }
    }

    pub fn from_achievements(game_id: u32, achievements: Vec<GameAchievement>) -> Self {
        let mut game_achievements = Self::new(game_id);
        for achievement in achievements {
            game_achievements.add_achievement(achievement);
        }
        game_achievements
    }

    pub fn add_achievement(&mut self, achievement: GameAchievement) {
        self.achievements
            .insert(achievement.id().to_string(), achievement);
    }

    pub fn achievements(&self) -> &HashMap<String, GameAchievement> {
        &self.achievements
    }

    pub fn achievements_count(&self) -> usize {
        self.achievements.len()
    }

    pub fn achievement_progress_formatted(&self) -> String {
        format!(
            "{} / {}",
            self.achievements_unlocked_count(),
            self.achievements_count()
        )
    }

    pub fn achievements_unlocked_count(&self) -> usize {
        self.achievements
            .iter()
            .filter(|achievement| achievement.1.is_unlocked())
            .count()
    }

    pub fn achievement_ids(&self) -> Vec<String> {
        self.achievements.keys().cloned().collect()
    }

    pub fn achievements_unlocked_ids(&self) -> Vec<String> {
        self.achievements
            .iter()
            .filter(|achievement| achievement.1.is_unlocked())
            .map(|achievement| achievement.0.clone())
            .collect()
    }

    pub fn achievements_locked_ids(&self) -> Vec<String> {
        self.achievements
            .iter()
            .filter(|achievement| !achievement.1.is_unlocked())
            .map(|achievement| achievement.0.clone())
            .collect()
    }

    pub fn achievements_by_unlocked_date(&self) -> Vec<GameAchievementUnlocked> {
        let mut achievements = self
            .achievements
            .values()
            .filter_map(|achievement| match achievement {
                GameAchievement::Unlocked(achievement) => Some(achievement.clone()),
                _ => None,
            })
            .collect::<Vec<GameAchievementUnlocked>>();

        achievements.sort_by(|a, b| b.unlocked_date().cmp(a.unlocked_date()));

        achievements
    }

    pub fn achievements_locked(&self) -> Vec<GameAchievementLocked> {
        self.achievements
            .values()
            .filter_map(|achievement| match achievement {
                GameAchievement::Locked(achievement) => Some(achievement.clone()),
                _ => None,
            })
            .collect()
    }
}
