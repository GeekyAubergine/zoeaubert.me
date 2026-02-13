use crate::domain::models::albums::album::Album;
use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::book::Book;
use crate::domain::models::games::steam::{SteamGame, SteamGameAchievementUnlocked};
use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::media::Media;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::movie::Movie;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::{
    TimelineEvent, TimelineEventGameAchievementUnlock, TimelineEventPost, TimelineEventReview,
};
use crate::domain::models::tv_show::TvShow;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::partials::md::{MarkdownMediaOption, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::tag::render_tags;
use chrono::{DateTime, Utc};
use hypertext::prelude::*;

use crate::domain::models::image::Image;

pub fn render_timline_events_list<'l>(events: &'l [&TimelineEvent]) -> impl Renderable + 'l {
    maud! {
        ul class="timeline-events-list" {
            @for event in events {
                @match event {
                    TimelineEvent::Post(post) => @match post {
                        TimelineEventPost::BlogPost(post) => (render_blog_post(post)),
                        TimelineEventPost::MicroPost(post) => (render_micro_post(post))
                        TimelineEventPost::MastodonPost(post) => (render_mastodon_post(post)),
                    },
                    TimelineEvent::Review(review) => @match review {
                        TimelineEventReview::BookReview { book, source, .. } => (render_book_review(book, source)),
                        TimelineEventReview::MovieReview { movie, source, .. } => (render_movie_review(movie, source)),
                        TimelineEventReview::TvShowReview { tv_show, source, .. } => (render_tv_show_review(tv_show, source)),
                    },
                    TimelineEvent::GameAchievementUnlock(achievement) => @match achievement {
                        TimelineEventGameAchievementUnlock::SteamAchievementUnlocked {
                            game,
                            achievement,
                        } => {
                            (render_game_achievement(game, achievement))
                        },
                    },
                    TimelineEvent::Album(album) => (render_album(album)),
                    TimelineEvent::AlbumPhoto { photo, .. } => (render_album_photo(photo)),
                }
                hr;
            }
        }
    }
}

fn render_post<'l>(
    slug: Slug,
    date: &'l DateTime<Utc>,
    content: impl Renderable + 'l,
    media: Option<Vec<Media>>,
    tags: Option<&'l Vec<Tag>>,
    side_image: Option<&'l Image>,
) -> impl Renderable + 'l {
    maud! {
        @match side_image {
            None => {
                li {
                    a class="date" href=(slug.relative_string()) {
                        time class="date" datetime=(date.datetime()) {
                            (format!("{} →", date.month_as_word()))
                        }
                    }
                    div class="content" {
                        (content)
                    }
                    @if let Some(media) = &media {
                        a href=(slug.relative_string()) {
                            (render_media_grid(media, &MediaGripOptions::for_list()))
                        }
                    }
                    @if let Some(tags) = &tags {
                        (render_tags(&tags, Some(5)))
                    }
                }
            },
            Some(side_image) => {
                li {
                    a class="date" href=(slug.relative_string()) {
                        time class="date" datetime=(date.datetime()) {
                            (format!("{} →", date.month_as_word()))
                        }
                    }
                    div class="left-right" {
                        div class="left" {
                            div class="content" {
                                (content)
                            }
                            @if let Some(media) = &media {
                                (render_media_grid(media, &MediaGripOptions::for_list()))
                            }
                            @if let Some(tags) = &tags {
                                (render_tags(&tags, Some(5)))
                            }
                        }
                        div class="right" {
                            a href=(slug.relative_string()) {
                                (side_image.render_large())
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_blog_post<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            a class="blog-title" href=(post.slug.relative_string()) {
                (&post.title)
            }
            p { (post.description )}
        }
    };

    render_post(
        post.slug.clone(),
        &post.date,
        content,
        None,
        Some(&post.tags),
        None,
    )
}

pub fn render_micro_post<'l>(post: &'l MicroPost) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            (md(&post.content(), MarkdownMediaOption::NoMedia))
        }
    };

    render_post(
        post.slug.clone(),
        &post.date,
        content,
        Some(post.media().clone()),
        Some(&post.tags),
        None,
    )
}

pub fn render_mastodon_post<'l>(post: &'l MastodonPost) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            (md(&post.content(), MarkdownMediaOption::NoMedia))
        }
    };

    render_post(
        post.slug(),
        post.created_at(),
        content,
        Some(post.media().clone()),
        Some(post.tags()),
        None,
    )
}

pub fn render_book_review<'l>(book: &'l Book, source: &'l ReviewSource) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            (md(&source.content(), MarkdownMediaOption::NoMedia))
        }
    };

    render_post(
        source.slug(),
        source.date(),
        content,
        None,
        Some(source.tags()),
        Some(&book.cover),
    )
}

pub fn render_movie_review<'l>(movie: &'l Movie, source: &'l ReviewSource) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            (md(&source.content(), MarkdownMediaOption::NoMedia))
        }
    };

    render_post(
        source.slug(),
        source.date(),
        content,
        None,
        Some(source.tags()),
        Some(&movie.poster),
    )
}

pub fn render_tv_show_review<'l>(
    tv_show: &'l TvShow,
    source: &'l ReviewSource,
) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            (md(&source.content(), MarkdownMediaOption::NoMedia))
        }
    };

    render_post(
        source.slug(),
        source.date(),
        content,
        None,
        Some(source.tags()),
        Some(&tv_show.poster),
    )
}

pub fn render_game_achievement<'l>(
    game: &'l SteamGame,
    achievement: &'l SteamGameAchievementUnlocked,
) -> impl Renderable + 'l {
    let content = maud! {
        a class="game-title" href=(game.slug().relative_string()) {
            (&game.name)
        }
        p { (format!("Unlocked {}", achievement.display_name)) }
        p { (achievement.description )}
    };

    render_post(
        game.slug(),
        &achievement.unlocked_date,
        content,
        None,
        None,
        Some(&achievement.image),
    )
}

pub fn render_album<'l>(album: &'l Album) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            a class="album-title" href=(album.slug.relative_string()) {
                (&album.title)
            }
        }
    };

    render_post(
        album.slug.clone(),
        &album.date,
        content,
        Some(
            album
                .cover_images()
                .iter()
                .map(|i| i.clone().into())
                .collect(),
        ),
        None,
        None,
    )
}

pub fn render_album_photo<'l>(photo: &'l AlbumPhoto) -> impl Renderable + 'l {
    let content = maud! {
        div class="prose" {
            a class="album-photo-description" href=(photo.slug.relative_string()) {
                p { (&photo.description) }
            }
        }
    };

    render_post(
        photo.slug.clone(),
        &photo.date,
        content,
        Some(vec![photo.image.clone().into()]),
        None,
        None,
    )
}
