use crate::domain::models::data::Data;
use crate::domain::models::image::Image;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::prelude::*;
use crate::renderer::{RenderTasks, RenderTask};
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::partials::page::{PageOptions, render_page},
};

const ITEM_COUNT: usize = 5;

struct InterestElement<'l> {
    sub_text: Option<String>,
    image: &'l Image,
    link: Slug,
}

pub fn render_interests_page<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    let games = data
        .games
        .find_by_most_recently_played()
        .iter()
        .map(|game| InterestElement {
            sub_text: None,
            image: game.image(),
            link: game.slug(),
        })
        .take(6)
        .collect::<Vec<InterestElement<'d>>>();

    let lego = data
        .lego
        .find_all_sets()
        .iter()
        .map(|set| InterestElement {
            sub_text: None,
            image: &set.image,
            link: Slug::new("/interests/lego/"),
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'d>>>();

    let books = data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::BookReview {
                review,
                book,
                source,
            }) => Some(InterestElement {
                sub_text: Some(format!("{}/5", review.score)),
                image: &book.cover,
                link: source.slug(),
            }),
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'d>>>();

    let movies = data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::MovieReview {
                review,
                movie,
                source,
            }) => Some(InterestElement {
                sub_text: Some(format!("{}/5", review.score)),
                image: &movie.poster,
                link: source.slug(),
            }),
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'d>>>();

    let tv_shows = data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::TvShowReview {
                review,
                tv_show,
                source,
            }) => Some(InterestElement {
                sub_text: Some(review.score_text()),
                image: &tv_show.poster,
                link: source.slug(),
            }),
            _ => None,
        })
        .take(ITEM_COUNT)
        .collect::<Vec<InterestElement<'d>>>();

    tasks.add(RenderInterestsPageTask {
        games,
        lego,
        books,
        movies,
        tv_shows,
    });
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

struct RenderInterestsPageTask<'l> {
    games: Vec<InterestElement<'l>>,
    lego: Vec<InterestElement<'l>>,
    books: Vec<InterestElement<'l>>,
    movies: Vec<InterestElement<'l>>,
    tv_shows: Vec<InterestElement<'l>>,
}

impl<'l> RenderTask for RenderInterestsPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(Slug::new("/interests"), Some("Interests".to_string()), None);

        let slug = page.slug.clone();

        let content = maud! {
            (render_interest_strip("Games", "Games →", "/interests/games/",  &self.games, "games"))
            (render_interest_strip("Lego", "Lego Sets →", "/interests/lego/",  &self.lego, "lego"))
            (render_interest_strip("Books", "Book Reviews →", "/tags/books/",  &self.books, "books"))
            (render_interest_strip("Movies", "Movie Reviews →", "/tags/movies/",  &self.movies, "movies"))
            (render_interest_strip("TV", "TV Reviews →", "/tags/tv/",  &self.tv_shows, "tv_shows"))
        };

        let options = PageOptions::new().with_main_class("interests-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
