use std::collections::HashMap;

use crate::domain::models::{
    blog_post::BlogPost,
    book::Book,
    mastodon_post::MastodonPost,
    micro_post::MicroPost,
    review::{book_review::BookReview, review_source::ReviewSource},
    tag::Tag,
};

use bitflags::bitflags;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TimelineEventPost {
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
}

#[derive(Debug, Clone)]
pub enum TimelineEvent {
    Post(TimelineEventPost),
    BookReview { review: BookReview, book: Book },
}

impl TimelineEvent {
    pub fn key(&self) -> String {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => post.slug.to_string(),
                TimelineEventPost::MicroPost(post) => post.slug.to_string(),
                TimelineEventPost::MastodonPost(post) => post.slug().to_string(),
            },
            TimelineEvent::BookReview { review, book } => review.source.slug().to_string(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => &post.date,
                TimelineEventPost::MicroPost(post) => &post.date,
                TimelineEventPost::MastodonPost(post) => &post.created_at(),
            },
            TimelineEvent::BookReview { review, book } => review.source.date(),
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => &post.tags,
                TimelineEventPost::MicroPost(post) => &post.tags,
                TimelineEventPost::MastodonPost(post) => post.tags(),
            },
            TimelineEvent::BookReview { review, book } => review.source.tags(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TimelineEvents {
    events_by_date: Vec<TimelineEvent>,
}

impl TimelineEvents {
    pub fn from_events(events: Vec<TimelineEvent>) -> Self {
        // Seems redundant, but prevents weird duplicates
        let mut events_map = events
            .into_iter()
            .map(|event| (event.key(), event))
            .collect::<HashMap<String, TimelineEvent>>();

        let mut events = events_map.into_values().collect::<Vec<TimelineEvent>>();

        events.sort_by(|a, b| b.date().cmp(&a.date()));

        Self {
            events_by_date: events,
        }
    }

    pub fn all_by_date(&self) -> &Vec<TimelineEvent> {
        &self.events_by_date
    }
}

impl From<ReviewSource> for TimelineEvent {
    fn from(value: ReviewSource) -> Self {
        match value {
            ReviewSource::MicroPost(post) => {
                TimelineEvent::Post(TimelineEventPost::MicroPost(post))
            }
            ReviewSource::MastodonPost(post) => {
                TimelineEvent::Post(TimelineEventPost::MastodonPost(post))
            }
        }
    }
}
