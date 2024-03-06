use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAchievement {
    id: String,
    display_name: String,
    description: String,
    image_unlocked_url: String,
    image_locked_url: String,
    unlocked_date: Option<DateTime<Utc>>,
}

impl GameAchievement {
    pub fn new(
        id: String,
        display_name: String,
        description: String,
        image_unlocked_url: String,
        image_locked_url: String,
        unlocked_date: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            display_name,
            description,
            image_unlocked_url,
            image_locked_url,
            unlocked_date,
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

    pub fn image_locked_url(&self) -> &str {
        &self.image_locked_url
    }

    pub fn unlocked_date(&self) -> Option<&DateTime<Utc>> {
        self.unlocked_date.as_ref()
    }

    pub fn is_unlocked(&self) -> bool {
        self.unlocked_date.is_some()
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
        last_played: u32,
        link_url: String,
        achievements: HashMap<String, GameAchievement>,
    ) -> Self {
        let last_played = match DateTime::from_timestamp(last_played as i64 * 1000, 0) {
            Some(date) => date,
            None => Utc::now(),
        };

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

    pub fn achievements_by_unlocked_date(&self) -> Vec<String> {
        let mut achievements = self
            .achievements
            .iter()
            .filter(|achievement| achievement.1.is_unlocked())
            .collect::<Vec<(&String, &GameAchievement)>>();

        achievements.sort_by(|a, b| a.1.unlocked_date().cmp(&b.1.unlocked_date()));

        achievements
            .iter()
            .map(|achievement| achievement.0.clone())
            .collect()
    }

    pub fn set_achievements(&mut self, achievements: HashMap<String, GameAchievement>) {
        self.achievements = achievements;
    }
}
