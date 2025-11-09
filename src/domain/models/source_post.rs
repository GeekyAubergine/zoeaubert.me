use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::albums::{album::Album, album_photo::AlbumPhoto};

use super::{
    blog_post::BlogPost,
    mastodon_post::MastodonPost,
    media::Media,
    micro_post::MicroPost,
    movie::{Movie, MovieReview},
    page::Page,
    slug::Slug,
    tag::Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourcePost {
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
}

impl SourcePost {
    pub fn slug(&self) -> Slug {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.clone(),
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
        }
    }

    pub fn link(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.slug.relative_string(),
            Self::MicroPost(micro_post) => micro_post.slug.relative_string(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().relative_string(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Self::BlogPost(blog_post) => &blog_post.date,
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            Self::BlogPost(blog_post) => blog_post.media.clone(),
            Self::MicroPost(micro_post) => micro_post.media.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.media(),
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            Self::BlogPost(blog_post) => &blog_post.tags,
            Self::MicroPost(micro_post) => &micro_post.tags,
            Self::MastodonPost(mastodon_post) => &mastodon_post.tags(),
        }
    }

    pub fn content(&self) -> String {
        match self {
            Self::BlogPost(blog_post) => blog_post.content.to_string(),
            Self::MicroPost(micro_post) => micro_post.content.to_string(),
            Self::MastodonPost(mastodon_post) => mastodon_post.content().to_string(),
        }
    }

    pub fn page(&self) -> Page {
        match self {
            Self::BlogPost(blog_post) => blog_post.page(),
            Self::MicroPost(micro_post) => micro_post.page(),
            Self::MastodonPost(mastodon_post) => mastodon_post.page(),
        }
    }
}

impl From<BlogPost> for SourcePost {
    fn from(blog_post: BlogPost) -> Self {
        Self::BlogPost(blog_post)
    }
}

impl From<MicroPost> for SourcePost {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<MastodonPost> for SourcePost {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}
