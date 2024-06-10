use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{prelude::*, utils::{FormatDate, FormatNumber}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAchievementUnlocked {
    id: String,
    display_name: String,
    description: String,
    image_unlocked_url: String,
    unlocked_date: DateTime<Utc>,
    global_unlocked_percentage: f32,
}

impl GameAchievementUnlocked {
    pub fn new(
        id: String,
        display_name: String,
        description: String,
        image_unlocked_url: String,
        unlocked_date: DateTime<Utc>,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            display_name,
            description,
            image_unlocked_url,
            unlocked_date,
            global_unlocked_percentage,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn image_unlocked_url(&self) -> &str {
        &self.image_unlocked_url
    }

    pub fn unlocked_date(&self) -> &DateTime<Utc> {
        &self.unlocked_date
    }

    pub fn global_unlocked_percentage(&self) -> f32 {
        self.global_unlocked_percentage
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAchievementLocked {
    id: String,
    display_name: String,
    description: String,
    image_locked_url: String,
    global_unlocked_percentage: f32,
}

impl GameAchievementLocked {
    pub fn new(
        id: String,
        display_name: String,
        description: String,
        image_locked_url: String,
        global_unlocked_percentage: f32,
    ) -> Self {
        Self {
            id,
            display_name,
            description,
            image_locked_url,
            global_unlocked_percentage,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn image_locked_url(&self) -> &str {
        &self.image_locked_url
    }

    pub fn global_unlocked_percentage(&self) -> f32 {
        self.global_unlocked_percentage
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameAchievement {
    #[serde(rename = "unlocked")]
    Unlocked(GameAchievementUnlocked),
    #[serde(rename = "locked")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    id: u32,
    name: String,
    header_image_url: String,
    playtime: u32,
    last_played: DateTime<Utc>,
    link_url: String,
    achievements: HashMap<String, GameAchievement>,
}

impl Game {
    pub fn new(
        id: u32,
        name: String,
        header_image_url: String,
        playtime: u32,
        last_played: DateTime<Utc>,
        link_url: String,
        achievements: HashMap<String, GameAchievement>,
    ) -> Self {
        Self {
            id,
            name,
            header_image_url,
            playtime,
            last_played,
            link_url,
            achievements,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn header_image_url(&self) -> &str {
        &self.header_image_url
    }

    pub fn playtime(&self) -> u32 {
        self.playtime
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime() as f32 / 60.0
    }

    pub fn last_played(&self) -> &DateTime<Utc> {
        &self.last_played
    }

    pub fn link_url(&self) -> &str {
        &self.link_url
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

    pub fn achievments_locked(&self) -> Vec<GameAchievementLocked> {
        self.achievements
            .values()
            .filter_map(|achievement| match achievement {
                GameAchievement::Locked(achievement) => Some(achievement.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn set_achievements(&mut self, achievements: HashMap<String, GameAchievement>) {
        self.achievements = achievements;
    }
}
