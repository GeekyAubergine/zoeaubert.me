use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::{
    mastodon_post::MastodonPost, media::Media, micro_post::MicroPost, page::Page, slug::Slug,
    tag::Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewSource {
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
}

impl ReviewSource {
    pub fn slug(&self) -> Slug {
        match self {
            Self::MicroPost(micro_post) => micro_post.slug.clone(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug(),
        }
    }

    pub fn link(&self) -> String {
        match self {
            Self::MicroPost(micro_post) => micro_post.slug.relative_string(),
            Self::MastodonPost(mastodon_post) => mastodon_post.slug().relative_string(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Self::MicroPost(micro_post) => &micro_post.date,
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
        }
    }

    pub fn media(&self) -> &Vec<Media> {
        match self {
            Self::MicroPost(micro_post) => &micro_post.media,
            Self::MastodonPost(mastodon_post) => mastodon_post.media(),
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            Self::MicroPost(micro_post) => &micro_post.tags,
            Self::MastodonPost(mastodon_post) => &mastodon_post.tags(),
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::MicroPost(micro_post) => &micro_post.content,
            Self::MastodonPost(mastodon_post) => mastodon_post.content(),
        }
    }

    pub fn page(&self) -> Page {
        match self {
            Self::MicroPost(micro_post) => micro_post.page(),
            Self::MastodonPost(mastodon_post) => mastodon_post.page(),
        }
    }
}

impl From<MicroPost> for ReviewSource {
    fn from(micro_post: MicroPost) -> Self {
        Self::MicroPost(micro_post)
    }
}

impl From<MastodonPost> for ReviewSource {
    fn from(mastodon_post: MastodonPost) -> Self {
        Self::MastodonPost(mastodon_post)
    }
}
