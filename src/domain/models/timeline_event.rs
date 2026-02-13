use std::collections::HashMap;

use crate::domain::models::{
    albums::{album::Album, album_photo::AlbumPhoto},
    blog_post::BlogPost,
    book::Book,
    games::steam::{SteamGame, SteamGameAchievementUnlocked},
    mastodon_post::MastodonPost,
    micro_post::MicroPost,
    movie::Movie,
    review::{
        book_review::BookReview, movie_review::MovieReview, review_source::ReviewSource,
        tv_show_review::TvShowReview,
    },
    tag::Tag,
    tv_show::TvShow,
};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TimelineEventPost {
    BlogPost(BlogPost),
    MicroPost(MicroPost),
    MastodonPost(MastodonPost),
}

#[derive(Debug, Clone)]
pub enum TimelineEventReview {
    BookReview {
        review: BookReview,
        book: Book,
        source: ReviewSource,
    },
    MovieReview {
        review: MovieReview,
        movie: Movie,
        source: ReviewSource,
    },
    TvShowReview {
        review: TvShowReview,
        tv_show: TvShow,
        source: ReviewSource,
    },
}

#[derive(Debug, Clone)]
pub enum TimelineEventGameAchievementUnlock {
    SteamAchievementUnlocked {
        game: SteamGame,
        achievement: SteamGameAchievementUnlocked,
    },
}

#[derive(Debug, Clone)]
pub enum TimelineEvent {
    Post(TimelineEventPost),
    Review(TimelineEventReview),
    GameAchievementUnlock(TimelineEventGameAchievementUnlock),
    Album(Album),
    AlbumPhoto { album: Album, photo: AlbumPhoto },
}

impl TimelineEvent {
    pub fn key(&self) -> String {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => post.slug.to_string(),
                TimelineEventPost::MicroPost(post) => post.slug.to_string(),
                TimelineEventPost::MastodonPost(post) => post.slug().to_string(),
            },
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::BookReview { source, .. } => source.slug().to_string(),
                TimelineEventReview::MovieReview { source, .. } => source.slug().to_string(),
                TimelineEventReview::TvShowReview { source, .. } => source.slug().to_string(),
            },
            TimelineEvent::GameAchievementUnlock(achievement) => match achievement {
                TimelineEventGameAchievementUnlock::SteamAchievementUnlocked { game, .. } => {
                    game.slug().to_string()
                }
            },
            TimelineEvent::Album(alumb) => alumb.slug.to_string(),
            TimelineEvent::AlbumPhoto { photo, .. } => photo.slug.to_string(),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => &post.date,
                TimelineEventPost::MicroPost(post) => &post.date,
                TimelineEventPost::MastodonPost(post) => post.created_at(),
            },
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::BookReview { source, .. } => source.date(),
                TimelineEventReview::MovieReview { source, .. } => source.date(),
                TimelineEventReview::TvShowReview { source, .. } => source.date(),
            },
            TimelineEvent::GameAchievementUnlock(achievement) => match achievement {
                TimelineEventGameAchievementUnlock::SteamAchievementUnlocked {
                    achievement,
                    ..
                } => &achievement.unlocked_date,
            },
            TimelineEvent::Album(album) => &album.date,
            TimelineEvent::AlbumPhoto { photo, .. } => &photo.date,
        }
    }

    pub fn tags(&self) -> Option<&Vec<Tag>> {
        match self {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => Some(&post.tags),
                TimelineEventPost::MicroPost(post) => Some(&post.tags),
                TimelineEventPost::MastodonPost(post) => Some(post.tags()),
            },
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::BookReview { source, .. } => Some(source.tags()),
                TimelineEventReview::MovieReview { source, .. } => Some(source.tags()),
                TimelineEventReview::TvShowReview { source, .. } => Some(source.tags()),
            },
            TimelineEvent::GameAchievementUnlock(_) => None,
            TimelineEvent::Album(_) => None,
            TimelineEvent::AlbumPhoto { photo, .. } => Some(&photo.tags),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct EventKey {
    key: String,
    date: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct TimelineEvents {
    events_by_date: Vec<TimelineEvent>,
}

impl TimelineEvents {
    pub fn from_events(events: Vec<TimelineEvent>) -> Self {
        // Seems redundant, but prevents weird duplicates
        let events_map = events
            .into_iter()
            .map(|event| {
                (
                    EventKey {
                        key: event.key(),
                        date: *event.date(),
                    },
                    event,
                )
            })
            .collect::<HashMap<EventKey, TimelineEvent>>();

        let mut events = events_map.into_values().collect::<Vec<TimelineEvent>>();

        events.sort_by(|a, b| b.date().cmp(a.date()));

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
