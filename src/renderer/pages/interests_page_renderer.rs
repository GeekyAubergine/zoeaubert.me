use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::{image::Image};
use crate::prelude::*;
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::{
        RendererContext,
        partials::page::{PageOptions, render_page},
    },
};

const ITEM_COUNT: usize = 5;

struct InterestElement<'l> {
    sub_text: Option<String>,
    image: &'l Image,
    link: Slug,
}

pub fn render_interests_page<'l>(context: &'l RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/interests"), Some("Interests".to_string()), None);

    let slug = page.slug.clone();

    let games = context
        .data
        .games
        .find_by_most_recently_played()
        .iter()
        .map(|game| InterestElement {
            sub_text: None,
            image: game.image(),
            link: game.slug(),
        })
        .take(6)
        .collect::<Vec<InterestElement<'l>>>();

    let lego = context
        .data
        .lego
        .find_all_sets()
        .iter()
        .map(|set| InterestElement {
            sub_text: None,
            image: &set.image,
            link: Slug::new("/interests/lego/"),
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'l>>>();

    let books = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::BookReview {
                    review,
                    book,
                    source,
                } => Some(InterestElement {
                    sub_text: Some(format!("{}/5", review.score)),
                    image: &book.cover,
                    link: source.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'l>>>();

    let movies = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::MovieReview {
                    review,
                    movie,
                    source,
                } => Some(InterestElement {
                    sub_text: Some(format!("{}/5", review.score)),
                    image: &movie.poster,
                    link: source.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'l>>>();

    let tv_shows = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(review) => match review {
                TimelineEventReview::TvShowReview {
                    review,
                    tv_show,
                    source,
                } => Some(InterestElement {
                    sub_text: Some(review.score_text()),
                    image: &tv_show.poster,
                    link: source.slug(),
                }),
                _ => None,
            },
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'l>>>();

    let content = maud! {
        (render_interest_strip("Games", "Games →", "/interests/games/",  &games, "games"))
        (render_interest_strip("Lego", "Lego Sets →", "/interests/lego/",  &lego, "lego"))
        (render_interest_strip("Books", "Book Reviews →", "/tags/books/",  &books, "books"))
        (render_interest_strip("Movies", "Movie Reviews →", "/tags/movies/",  &movies, "movies"))
        (render_interest_strip("TV", "TV Reviews →", "/tags/tv/",  &tv_shows, "tv_shows"))
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
        section class=(section_class) {
            h2 { (title) }
            ul {
                @for item in items {
                    li {
                        a href=(item.link.relative_string()) {
                            (item.image.render_small())
                            @if let Some(sub_text) = &item.sub_text {
                                p class="sub-text" { (sub_text) }
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
