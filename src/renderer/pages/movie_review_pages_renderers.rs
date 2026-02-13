use hypertext::prelude::*;

use crate::domain::models::movie::Movie;
use crate::domain::models::page::Page;
use crate::domain::models::review::movie_review::MovieReview;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};

// TODO Clicking on cover image should link you to tmdb page

pub fn render_move_review_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::MovieReview {
                review,
                movie,
                source,
            }) => Some((review, movie, source)),
            _ => None,
        })
        .collect::<Vec<(&MovieReview, &Movie, &ReviewSource)>>();

    for (_, movie, source) in posts {
        render_movie_review_page(context, movie, source)?;
    }

    Ok(())
}

pub fn render_movie_review_page(
    context: &RendererContext,
    movie: &Movie,
    source: &ReviewSource,
) -> Result<()> {
    let content = maud! {
        article {
            (md(&source.content(), md::MarkdownMediaOption::NoMedia))
        }
    };

    let options = PageOptions::new()
        .with_main_class("movie-review-post-page")
        .use_date_as_title()
        .with_image(&movie.poster);

    let page = Page::new(source.slug().clone(), None, None)
        .with_date(*source.date())
        .with_tags(source.tags().clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context
        .renderer
        .render_page(&source.slug(), &rendered, Some(*source.date()))
}
