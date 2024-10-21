use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    blog_post::BlogPost, mastodon_post::MastodonPost, media::Media, micro_post::MicroPost,
    slug::Slug, tag::Tag,
};

#[derive(Debug, Clone)]
pub enum OmniPost {
    // StatusLol(StatusLolPost),
    // UnlockedGameAchievement {
    //     game: Game,
    //     achievement: GameAchievementUnlocked,
    // },
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
}

impl OmniPost {
    pub fn slug(&self) -> Slug {
        match self {
            // Self::StatusLol(status_lol) => status_lol.slug().to_owned(),
            // Self::UnlockedGameAchievement { achievement, .. } => achievement.slug().to_owned(),
            Self::BlogPost(blog_post) => blog_post.slug.clone(),
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
        }
    }

    // pub fn key(&self) -> String {
    //     match self {
    //         // Self::StatusLol(status_lol) => status_lol.id().to_owned(),
    //         // Self::UnlockedGameAchievement { achievement, .. } => achievement.id().to_owned(),
    //         Self::BlogPost(blog_post) => blog_post.slug().to_owned(),
    //         // Self::MicroPost(micro_post) => micro_post.slug.to_owned(),
    //         // Self::MastodonPost(mastodon_post) => mastodon_post.id().to_owned(),
    //     }
    // }

    pub fn link(&self) -> String {
        match self {
            // Self::StatusLol(status_lol) => status_lol.permalink().to_owned(),
            // Self::UnlockedGameAchievement { game, .. } => format!("/interests/games/{}", game.id()),
            Self::BlogPost(blog_post) => blog_post.slug.relative_link(),
            Self::MicroPost(micro_post) => micro_post.slug.relative_link(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().relative_link(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            // Self::StatusLol(status_lol) => status_lol.date(),
            // Self::UnlockedGameAchievement { achievement, .. } => achievement.unlocked_date(),
            Self::BlogPost(blog_post) => &blog_post.date,
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
        }
    }

    pub fn media(&self) -> &Vec<Media> {
        match self {
            // Self::StatusLol(status_lol) => vec![],
            // Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => &blog_post.media,
            Self::MicroPost(micro_post) => &micro_post.media,
            Self::MastodonPost(mastodon_post) => mastodon_post.media(), //     .media()
                                                                        //     .iter()
                                                                        //     .map(|media| media.uuid())
                                                                        //     .cloned()
                                                                        //     .collect(),
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            // Self::StatusLol(status_lol) => status_lol.tags.clone(),
            // Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => blog_post.tags.clone(),
            Self::MicroPost(micro_post) => micro_post.tags.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.tags().clone(),
        }
    }
}

impl From<BlogPost> for OmniPost {
    fn from(blog_post: BlogPost) -> Self {
        Self::BlogPost(blog_post)
    }
}

impl From<&BlogPost> for OmniPost {
    fn from(blog_post: &BlogPost) -> Self {
        Self::BlogPost(blog_post.clone())
    }
}

impl From<MicroPost> for OmniPost {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<&MicroPost> for OmniPost {
    fn from(micro_post: &MicroPost) -> Self {
        Self::MicroPost(micro_post.clone())
    }
}

impl From<MastodonPost> for OmniPost {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}

impl From<&MastodonPost> for OmniPost {
    fn from(mastodon_post: &MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post.clone())
    }
}
