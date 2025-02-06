use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    album::{Album, AlbumPhoto},
    blog_post::BlogPost,
    mastodon_post::MastodonPost,
    media::Media,
    micro_post::MicroPost,
    movie::{Movie, MovieReview},
    page::Page,
    slug::Slug,
    steam::{SteamGame, SteamGameAchievementUnlocked},
    tag::Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
    AlbumPhoto { album: Album, photo: AlbumPhoto },
    Album(Album),
}

impl Content {
    pub fn slug(&self) -> Slug {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.clone(),
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
            Self::AlbumPhoto { photo, .. } => photo.slug.clone(),
            Self::Album(album) => album.slug.clone(),
        }
    }

    pub fn link(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.relative_link(),
            Self::MicroPost(micro_post) => micro_post.slug.relative_link(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().relative_link(),
            Self::AlbumPhoto { photo, .. } => photo.slug.relative_link(),
            Self::Album(album) => album.slug.relative_link(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Self::BlogPost(blog_post) => &blog_post.date,
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
            Self::AlbumPhoto { photo, .. } => &photo.date,
            Self::Album(album) => &album.date,
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.media(),
            Self::AlbumPhoto { photo, .. } => {
                vec![photo.small_image.clone().into()]
            }
            Self::Album(_) => vec![], // It does it's own thing
        }
    }

    pub fn optimised_media(&self) -> Vec<Media> {
        match self {
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.optimised_media(),
            Self::AlbumPhoto { photo, .. } => {
                vec![photo.small_image.clone().into()]
            }
            Self::Album(_) => vec![], // It does it's own thing
        }
    }

    pub fn tags(&self) -> Vec<Tag> {
        match self {
            Self::BlogPost(blog_post) => blog_post.tags.clone(),
            Self::MicroPost(micro_post) => micro_post.tags.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.tags().clone(),
            Self::AlbumPhoto { photo, .. } => photo.tags.clone(),
            Self::Album(_) => vec![], // Don't want it in search
        }
    }

    pub fn last_updated_at(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::BlogPost(blog_post) => Some(&blog_post.updated_at),
            Self::MicroPost(micro_post) => micro_post.updated_at.as_ref(),
            Self::MastodonPost(mastodon_post) => Some(mastodon_post.updated_at()),
            Self::AlbumPhoto { photo, .. } => Some(&photo.updated_at),
            Self::Album(album) => Some(&album.updated_at),
        }
    }

    pub fn content(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.content.to_string(),
            Self::MicroPost(micro_post) => micro_post.content.to_string(),
            Self::MastodonPost(mastodon_post) => mastodon_post.content().to_string(),
            Self::AlbumPhoto { photo, .. } => photo.description.to_string(),
            Self::Album(album) => match &album.description {
                Some(description) => description.to_string(),
                None => "".to_string(),
            },
        }
    }

    pub fn page(&self) -> Page {
        match self {
            Self::BlogPost(blog_post) => blog_post.page(),
            Self::MicroPost(micro_post) => micro_post.page(),
            Self::MastodonPost(mastodon_post) => mastodon_post.page(),
            Self::AlbumPhoto { photo, .. } => photo.page(),
            Self::Album(album) => album.page(),
        }
    }
}

impl From<BlogPost> for Content {
    fn from(blog_post: BlogPost) -> Self {
        Self::BlogPost(blog_post)
    }
}

impl From<MicroPost> for Content {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<MastodonPost> for Content {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}

impl From<(Album, AlbumPhoto)> for Content {
    fn from((album, photo): (Album, AlbumPhoto)) -> Self {
        Self::AlbumPhoto { album, photo }
    }
}

impl From<(AlbumPhoto, Album)> for Content {
    fn from((photo, album): (AlbumPhoto, Album)) -> Self {
        Self::AlbumPhoto { album, photo }
    }
}

impl From<Album> for Content {
    fn from(album: Album) -> Self {
        Self::Album(album)
    }
}
