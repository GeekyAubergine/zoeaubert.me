use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::models::media::Media;

use super::{slug::Slug, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostNonSpoiler {
    id: String,
    original_uri: String,
    created_at: DateTime<Utc>,
    content: String,
    media: Vec<Media>,
    media_previews: Vec<Media>,
    tags: Vec<Tag>,
}

impl MastodonPostNonSpoiler {
    pub fn new(
        id: String,
        original_uri: String,
        created_at: DateTime<Utc>,
        content: String,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            id,
            original_uri,
            created_at,
            content,
            media: vec![],
            media_previews: vec![],
            tags,
        }
    }

    pub fn add_media(&mut self, media: Media, preview: Option<Media>) {
        self.media.push(media);
        if let Some(preview) = preview {
            self.media_previews.push(preview);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostSpoiler {
    pub id: String,
    pub original_uri: String,
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub media: Vec<Media>,
    pub media_previews: Vec<Media>,
    pub spoiler_text: String,
    pub tags: Vec<Tag>,
}

impl MastodonPostSpoiler {
    pub fn new(
        id: String,
        original_uri: String,
        created_at: DateTime<Utc>,
        content: String,
        spoiler_text: String,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            id,
            original_uri,
            created_at,
            content,
            media: vec![],
            media_previews: vec![],
            spoiler_text,
            tags,
        }
    }

    pub fn add_media(&mut self, media: Media, preview: Option<Media>) {
        self.media.push(media);
        if let Some(preview) = preview {
            self.media_previews.push(preview);
        }
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
            MastodonPost::NonSpoiler(post) => &post.id,
            MastodonPost::Spoiler(post) => &post.id,
        }
    }

    pub fn original_uri(&self) -> &str {
        match self {
            MastodonPost::NonSpoiler(post) => &post.original_uri,
            MastodonPost::Spoiler(post) => &post.original_uri,
        }
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        match self {
            MastodonPost::NonSpoiler(post) => &post.created_at,
            MastodonPost::Spoiler(post) => &post.created_at,
        }
    }

    pub fn content(&self) -> &str {
        match self {
            MastodonPost::NonSpoiler(post) => &post.content,
            MastodonPost::Spoiler(post) => &post.content,
        }
    }

    pub fn media(&self) -> &Vec<Media> {
        match self {
            MastodonPost::NonSpoiler(post) => &post.media,
            MastodonPost::Spoiler(post) => &post.media,
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            MastodonPost::NonSpoiler(post) => &post.tags,
            MastodonPost::Spoiler(post) => &post.tags,
        }
    }

    pub fn slug(&self) -> Slug {
        Slug::new(&format!("micros/{}", self.id()))
    }

    pub fn add_media(&mut self, media: Media, preview: Option<Media>) {
        match self {
            MastodonPost::NonSpoiler(post) => post.add_media(media, preview),
            MastodonPost::Spoiler(post) => post.add_media(media, preview),
        }
    }

    pub fn optimised_media(&self) -> Vec<Media> {
        match self {
            MastodonPost::NonSpoiler(post) => {
                if post.media_previews.is_empty() {
                    post.media.clone()
                } else {
                    post.media_previews.clone()
                }
            }
            MastodonPost::Spoiler(post) => {
                if post.media_previews.is_empty() {
                    post.media.clone()
                } else {
                    post.media_previews.clone()
                }
            }
        }
    }
}
