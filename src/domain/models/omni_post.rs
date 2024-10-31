use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    album::{Album, AlbumPhoto}, blog_post::BlogPost, mastodon_post::MastodonPost, media::Media,
    micro_post::MicroPost, slug::Slug, tag::Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OmniPost {
    // StatusLol(StatusLolPost),
    // UnlockedGameAchievement {
    //     game: Game,
    //     achievement: GameAchievementUnlocked,
    // },
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
    AlbumPhoto(AlbumPhoto),
    Album(Album),
}

impl OmniPost {
    pub fn slug(&self) -> Slug {
        match self {
            // Self::StatusLol(status_lol) => status_lol.slug().to_owned(),
            // Self::UnlockedGameAchievement { achievement, .. } => achievement.slug().to_owned(),
            Self::BlogPost(blog_post) => blog_post.slug.clone(),
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
            Self::AlbumPhoto(album_photo) => album_photo.slug.clone(),
            Self::Album(album) => album.slug.clone(),
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
            Self::AlbumPhoto(album_photo) => album_photo.slug.relative_link(),
            Self::Album(album) => album.slug.relative_link(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            // Self::StatusLol(status_lol) => status_lol.date(),
            // Self::UnlockedGameAchievement { achievement, .. } => achievement.unlocked_date(),
            Self::BlogPost(blog_post) => &blog_post.date,
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
            Self::AlbumPhoto(album_photo) => &album_photo.date,
            Self::Album(album) => &album.date,
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            // Self::StatusLol(status_lol) => vec![],
            // Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.media(),
            Self::AlbumPhoto(album_photo) => {
                vec![album_photo.small_image.clone().into()]
            },
            // It does it's own thing
            Self::Album(_) => vec![],
        }
    }

    pub fn optimised_media(&self) -> Vec<Media> {
        match self {
            // Self::StatusLol(status_lol) => vec![],
            // Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.optimised_media(),
            Self::AlbumPhoto(album_photo) => {
                vec![album_photo.small_image.clone().into()]
            }
            // It does it's own thing
            Self::Album(_) => vec![],
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            // Self::StatusLol(status_lol) => status_lol.tags.clone(),
            // Self::UnlockedGameAchievement { .. } => vec![],
            Self::BlogPost(blog_post) => blog_post.tags.clone(),
            Self::MicroPost(micro_post) => micro_post.tags.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.tags().clone(),
            Self::AlbumPhoto(album_photo) => album_photo.tags.clone(),
            Self::Album(album) => vec![],
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

impl From<AlbumPhoto> for OmniPost {
    fn from(album_photo: AlbumPhoto) -> Self {
        Self::AlbumPhoto(album_photo)
    }
}

impl From<&AlbumPhoto> for OmniPost {
    fn from(album_photo: &AlbumPhoto) -> Self {
        Self::AlbumPhoto(album_photo.clone())
    }
}

impl From<Album> for OmniPost {
    fn from(album: Album) -> Self {
        Self::Album(album)
    }
}

impl From<&Album> for OmniPost {
    fn from(album: &Album) -> Self {
        Self::Album(album.clone())
    }
}
