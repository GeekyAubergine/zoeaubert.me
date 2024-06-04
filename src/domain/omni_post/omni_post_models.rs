use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{
    games::games_models::{Game, GameAchievementUnlocked},
    models::tag::Tag,
    status_lol::status_lol_models::StatusLolPost,
};

#[derive(Debug, Clone)]
pub enum OmniPost {
    StatusLol(StatusLolPost),
    UnlockedGameAchievement {
        game: Game,
        achievement: GameAchievementUnlocked,
    },
}

impl OmniPost {
    pub fn key(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.key().to_owned(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.id().to_owned(),
        }
    }

    pub fn permalink(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.permalink().to_owned(),
            Self::UnlockedGameAchievement { game, .. } => format!("/interests/games/{}", game.id()),
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::StatusLol(status_lol) => status_lol.date(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.unlocked_date(),
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::StatusLol(status_lol) => vec![Tag::new("StatusLol")],
            Self::UnlockedGameAchievement { game, .. } => vec![Tag::new("Gaming")],
        }
    }
}
