use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{
    blog_posts::blog_post_models::BlogPost, games::games_models::{Game, GameAchievementUnlocked}, models::tag::Tag, status_lol::status_lol_models::StatusLolPost
};

#[derive(Debug, Clone)]
pub enum OmniPost {
    StatusLol(StatusLolPost),
    UnlockedGameAchievement {
        game: Game,
        achievement: GameAchievementUnlocked,
    },
    BlogPost(BlogPost),
}

impl OmniPost {
    pub fn key(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.key().to_owned(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.id().to_owned(),
            Self::BlogPost(blog_post) => blog_post.slug().to_owned(),
        }
    }

    pub fn permalink(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.permalink().to_owned(),
            Self::UnlockedGameAchievement { game, .. } => format!("/interests/games/{}", game.id()),
            Self::BlogPost(blog_post) => blog_post.permalink().to_owned(),
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::StatusLol(status_lol) => status_lol.date(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.unlocked_date(),
            Self::BlogPost(blog_post) => blog_post.date(),
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::StatusLol(status_lol) => vec![Tag::new("StatusLol")],
            Self::UnlockedGameAchievement { game, .. } => vec![Tag::new("Games")],
            Self::BlogPost(blog_post) => blog_post.tags().to_owned(),
        }
    }
}
