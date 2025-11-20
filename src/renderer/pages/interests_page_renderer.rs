use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::{image::Image, review::book_review::BookReview};
use crate::prelude::*;
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::{
        RendererContext,
        partials::page::{PageOptions, render_page},
    },
};

struct InterestElement<'l> {
    title: String,
    sub_text: Option<String>,
    image: &'l Image,
    link: Slug,
}

pub fn render_interests_page<'l>(context: &'l RendererContext) -> Result<()> {
    let projects = context.data.projects.find_all_by_rank_and_name();

    let page = Page::new(Slug::new("/interests"), Some("Interests".to_string()), None);

    let slug = page.slug.clone();

    let books = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::BookReview { review, book, .. } => Some(InterestElement {
                    title: book.title.clone(),
                    sub_text: Some(format!("{}/5", review.score)),
                    image: &book.cover,
                    link: book.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(4)
        .collect::<Vec<InterestElement<'l>>>();

    let movies = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::MovieReview { review, movie, .. } => Some(InterestElement {
                    title: movie.title.clone(),
                    sub_text: Some(format!("{}/5", review.score)),
                    image: &movie.poster,
                    link: movie.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(4)
        .collect::<Vec<InterestElement<'l>>>();

    let tv_shows = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::TvShowReview { review, tv_show, .. } => Some(InterestElement {
                    title: format!("{} - {}", tv_show.title, review.season_text()),
                    sub_text: Some(format!("{}/5", review.score_text())),
                    image: &tv_show.poster,
                    link: tv_show.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(4)
        .collect::<Vec<InterestElement<'l>>>();

    let content = maud! {
        (render_interest_strip("Books", "More book reviews →", "/interests/books/",  &books, "books"))
        (render_interest_strip("Movies", "More movie reviews →", "/interests/movies/",  &movies, "movies"))
        (render_interest_strip("TV", "More tv reviews →", "/interests/tv/",  &tv_shows, "tv_shows"))
    };

    let options = PageOptions::new().with_main_class("interests-page");

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

fn render_interest_strip<'l>(
    title: &'l str,
    more_text: &'l str,
    more_link: &'l str,
    items: &'l [InterestElement<'l>],
    section_class: &'l str,
) -> impl Renderable + 'l {
    maud! {
        section {
            h2 { (title) }
            ul {
                @for item in items {
                    li {
                        a href=(item.link.relative_string()) {
                            p class="title" { (item.title) }
                        }
                        a href=(item.link.relative_string()) {
                            div class="content" {
                                (item.image.render_small())
                            }
                        }
                    }
                }
            }
            a class="more-link" href=(more_link) {
                (more_text)
            }
        }
    }
}
