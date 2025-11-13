use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::book::Book;
use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::media::{Media, MediaDimensions};
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::movie::Movie;
use crate::domain::models::review::book_review::BookReview;
use crate::domain::models::review::movie_review::MovieReview;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{MarkdownMediaOption, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::tag::render_tags;
use chrono::{DateTime, Utc};
use hypertext::prelude::*;

use crate::domain::models::image::{Image, SizedImage};
use crate::renderer::{TemplateRenderResult, render_template};

pub fn render_timline_events_list<'l>(events: &'l [&TimelineEvent]) -> impl Renderable + 'l {
    maud! {
        ul class="timeline-events-list" {
            @for event in events {
                @match event {
                    TimelineEvent::Post(event) => @match event {
                        TimelineEventPost::BlogPost(post) => (render_blog_post(post)),
                        TimelineEventPost::MicroPost(post) => (render_micro_post(post))
                        TimelineEventPost::MastodonPost(post) => (render_mastodon_post(post)),
                    },
                    TimelineEvent::BookReview { review, book, source } => (render_book_review(review, book, source)),
                    TimelineEvent::MovieReview { review, movie, source } => (render_movie_review(review, movie, source)),
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
    tags: &'l Vec<Tag>,
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
                        (render_media_grid(media, &MediaGripOptions::for_list()))
                    }
                    (render_tags(&tags, Some(5)))
                }
            },
            Some(side_image) => {
                li class="left-right" {
                    div class="left" {
                        a class="date" href=(slug.relative_string()) {
                            time class="date" datetime=(date.datetime()) {
                                (format!("{} →", date.month_as_word()))
                            }
                        }
                        div class="content" {
                            (content)
                        }
                        @if let Some(media) = &media {
                            (render_media_grid(media, &MediaGripOptions::for_list()))
                        }
                        (render_tags(&tags, Some(5)))
                    }
                    div class="right" {
                        (side_image.render_large())
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
        &post.tags,
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
        &post.tags,
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
        post.tags(),
        None,
    )
}

pub fn render_book_review<'l>(
    review: &'l BookReview,
    book: &'l Book,
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
        source.tags(),
        Some(&book.cover),
    )
}

pub fn render_movie_review<'l>(
    review: &'l MovieReview,
    movie: &'l Movie,
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
        source.tags(),
        Some(&movie.poster),
    )
}
