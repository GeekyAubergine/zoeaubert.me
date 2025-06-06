use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::models::media::Media;

use super::{page::Page, slug::Slug, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPostNonSpoiler {
    id: String,
    original_uri: Url,
    created_at: DateTime<Utc>,
    content: String,
    media: Vec<Media>,
    media_previews: Vec<Media>,
    tags: Vec<Tag>,
    updated_at: DateTime<Utc>,
}

impl MastodonPostNonSpoiler {
    pub fn new(
        id: String,
        original_uri: Url,
        created_at: DateTime<Utc>,
        content: String,
        tags: Vec<Tag>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            original_uri,
            created_at,
            content,
            media: vec![],
            media_previews: vec![],
            tags,
            updated_at,
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
    pub original_uri: Url,
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub media: Vec<Media>,
    pub media_previews: Vec<Media>,
    pub spoiler_text: String,
    pub tags: Vec<Tag>,
    pub updated_at: DateTime<Utc>,
}

impl MastodonPostSpoiler {
    pub fn new(
        id: String,
        original_uri: Url,
        created_at: DateTime<Utc>,
        content: String,
        spoiler_text: String,
        tags: Vec<Tag>,
        updated_at: DateTime<Utc>,
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
            updated_at,
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

    pub fn original_uri(&self) -> &Url {
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

    pub fn media(&self) -> Vec<Media> {
        match self {
            MastodonPost::NonSpoiler(post) => post.media.clone(),
            MastodonPost::Spoiler(post) => post.media.clone(),
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

    pub fn updated_at(&self) -> &DateTime<Utc> {
        match self {
            MastodonPost::NonSpoiler(post) => &post.updated_at,
            MastodonPost::Spoiler(post) => &post.updated_at,
        }
    }

    pub fn page(&self) -> Page {
        let content = self.content().replace("<p>", "\n").replace("</p>", "\n");

        let lines = content.lines().collect::<Vec<&str>>();

        let lines = lines
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();

        let first_line = match lines.first() {
            Some(first) => Some(*first),
            None => None,
        };

        let second_line = match lines.get(1) {
            Some(second) => Some(*second),
            None => None,
        };

        let description = match (first_line, second_line) {
            (Some(first), Some(second)) => {
                if first.len() > 100 {
                    Some(first.to_string())
                } else {
                    Some(format!("{}\n{}", first, second))
                }
            }
            (Some(first), None) => Some(first.to_string()),
            (None, _) => None,
        };

        let mut page = Page::new(self.slug().clone(), None, description)
            .with_date(*self.created_at())
            .with_tags(self.tags().clone());

        if let Some(first) = self.media().first() {
            match first {
                Media::Image(image) => {
                    page = page.with_image(image.clone().into());
                }
            }
        }

        page
    }
}
