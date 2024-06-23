use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{
    blog_posts::blog_post_models::BlogPost,
    games::games_models::{Game, GameAchievementUnlocked},
    micro_posts::micro_posts_models::MicroPost,
    microblog_archive::microblog_archive_models::MicroblogArchivePost,
    models::{media::Media, tag::Tag},
    status_lol::status_lol_models::StatusLolPost,
};

#[derive(Debug, Clone)]
pub enum OmniPost {
    StatusLol(StatusLolPost),
    UnlockedGameAchievement {
        game: Game,
        achievement: GameAchievementUnlocked,
    },
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MicroblogArchivePost(MicroblogArchivePost),
}

impl OmniPost {
    pub fn key(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.key().to_owned(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.id().to_owned(),
            Self::BlogPost(blog_post) => blog_post.slug().to_owned(),
            Self::MicroPost(micro_post) => micro_post.slug().to_owned(),
            Self::MicroblogArchivePost(microblog_archive_post) => {
                microblog_archive_post.slug().to_owned()
            }
        }
    }

    pub fn permalink(&self) -> String {
        match self {
            Self::StatusLol(status_lol) => status_lol.permalink().to_owned(),
            Self::UnlockedGameAchievement { game, .. } => format!("/interests/games/{}", game.id()),
            Self::BlogPost(blog_post) => blog_post.permalink().to_owned(),
            Self::MicroPost(micro_post) => micro_post.permalink().to_owned(),
            Self::MicroblogArchivePost(microblog_archive_post) => {
                microblog_archive_post.permalink().to_owned()
            }
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::StatusLol(status_lol) => status_lol.date(),
            Self::UnlockedGameAchievement { achievement, .. } => achievement.unlocked_date(),
            Self::BlogPost(blog_post) => blog_post.date(),
            Self::MicroPost(micro_post) => micro_post.date(),
            Self::MicroblogArchivePost(microblog_archive_post) => microblog_archive_post.date(),
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::StatusLol(status_lol) => vec![Tag::from_string("StatusLol")],
            Self::UnlockedGameAchievement { game, .. } => vec![Tag::from_string("Gaming")],
            Self::BlogPost(blog_post) => blog_post.tags().to_owned(),
            Self::MicroPost(micro_post) => micro_post.tags().to_owned(),
            Self::MicroblogArchivePost(microblog_archive_post) => {
                microblog_archive_post.tags().to_owned()
            }
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            Self::StatusLol(status_lol) => vec![],
            Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => blog_post.media().to_owned(),
            Self::MicroPost(micro_post) => vec![], // micro_post.media().to_owned(),
            Self::MicroblogArchivePost(microblog_archive_post) => vec![], // {
                                                    // microblog_archive_post.media().to_owned()
                                                    // }
        }
    }
}
