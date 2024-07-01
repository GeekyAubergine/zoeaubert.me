use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::media::Media;

use super::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostNonSpoiler {
    id: String,
    original_uri: String,
    created_at: DateTime<Utc>,
    content: String,
    media: Vec<Media>,
    reblogs_count: u32,
    favourites_count: u32,
    replies_count: u32,
    tags: Vec<Tag>,
}

impl MastodonPostNonSpoiler {
    pub fn new(
        id: String,
        original_uri: String,
        created_at: DateTime<Utc>,
        content: String,
        reblogs_count: u32,
        favourites_count: u32,
        replies_count: u32,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            id,
            original_uri,
            created_at,
            content,
            media: vec![],
            reblogs_count,
            favourites_count,
            replies_count,
            tags,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn original_uri(&self) -> &str {
        &self.original_uri
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }

    pub fn reblogs_count(&self) -> u32 {
        self.reblogs_count
    }

    pub fn favourites_count(&self) -> u32 {
        self.favourites_count
    }

    pub fn replies_count(&self) -> u32 {
        self.replies_count
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn add_media(&mut self, media: Media) {
        self.media.push(media);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostSpoiler {
    id: String,
    original_uri: String,
    created_at: DateTime<Utc>,
    content: String,
    media: Vec<Media>,
    reblogs_count: u32,
    favourites_count: u32,
    replies_count: u32,
    spoiler_text: String,
    tags: Vec<Tag>,
}

impl MastodonPostSpoiler {
    pub fn new(
        id: String,
        original_uri: String,
        created_at: DateTime<Utc>,
        content: String,
        reblogs_count: u32,
        favourites_count: u32,
        replies_count: u32,
        spoiler_text: String,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            id,
            original_uri,
            created_at,
            content,
            media: vec![],
            reblogs_count,
            favourites_count,
            replies_count,
            spoiler_text,
            tags,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn original_uri(&self) -> &str {
        &self.original_uri
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn media(&self) -> &Vec<Media> {
        &self.media
    }

    pub fn reblogs_count(&self) -> u32 {
        self.reblogs_count
    }

    pub fn favourites_count(&self) -> u32 {
        self.favourites_count
    }

    pub fn replies_count(&self) -> u32 {
        self.replies_count
    }

    pub fn spoiler_text(&self) -> &str {
        &self.spoiler_text
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn add_media(&mut self, media: Media) {
        self.media.push(media);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MastodonPost {
    NonSpoiler(MastodonPostNonSpoiler),
    Spoiler(MastodonPostSpoiler),
}

impl MastodonPost {
    pub fn id(&self) -> &str {
        match self {
            MastodonPost::NonSpoiler(post) => post.id(),
            MastodonPost::Spoiler(post) => post.id(),
        }
    }

    pub fn original_uri(&self) -> &str {
        match self {
            MastodonPost::NonSpoiler(post) => post.original_uri(),
            MastodonPost::Spoiler(post) => post.original_uri(),
        }
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        match self {
            MastodonPost::NonSpoiler(post) => post.created_at(),
            MastodonPost::Spoiler(post) => post.created_at(),
        }
    }

    pub fn content(&self) -> &str {
        match self {
            MastodonPost::NonSpoiler(post) => post.content(),
            MastodonPost::Spoiler(post) => post.content(),
        }
    }

    pub fn media(&self) -> &Vec<Media> {
        match self {
            MastodonPost::NonSpoiler(post) => post.media(),
            MastodonPost::Spoiler(post) => post.media(),
        }
    }

    pub fn reblogs_count(&self) -> u32 {
        match self {
            MastodonPost::NonSpoiler(post) => post.reblogs_count(),
            MastodonPost::Spoiler(post) => post.reblogs_count(),
        }
    }

    pub fn favourites_count(&self) -> u32 {
        match self {
            MastodonPost::NonSpoiler(post) => post.favourites_count(),
            MastodonPost::Spoiler(post) => post.favourites_count(),
        }
    }

    pub fn replies_count(&self) -> u32 {
        match self {
            MastodonPost::NonSpoiler(post) => post.replies_count(),
            MastodonPost::Spoiler(post) => post.replies_count(),
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            MastodonPost::NonSpoiler(post) => post.tags(),
            MastodonPost::Spoiler(post) => post.tags(),
        }
    }

    pub fn permalink(&self) -> String {
        format!("/micros/{}", self.id())
    }

    pub fn add_media(&mut self, media: Media) {
        match self {
            MastodonPost::NonSpoiler(post) => post.add_media(media),
            MastodonPost::Spoiler(post) => post.add_media(media),
        }
    }
}
