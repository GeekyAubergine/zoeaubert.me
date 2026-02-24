use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::movie::Movie;
use crate::domain::models::page::Page;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::prelude::*;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTask, RenderTasks};
use crate::services::page_renderer::PageRenderer;

// TODO Clicking on cover image should link you to tmdb page

pub fn render_movie_review_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::MovieReview { movie, source, .. }) => {
                Some((movie, source))
            }
            _ => None,
        })
        .for_each(|(movie, source)| tasks.add(RenderMovieReviewPageTask { movie, source }));
}

struct RenderMovieReviewPageTask<'l> {
    movie: &'l Movie,
    source: &'l ReviewSource,
}

impl<'l> RenderTask for RenderMovieReviewPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let source = self.source;

        let content = maud! {
            article {
                (md(&source.content(), md::MarkdownMediaOption::NoMedia))
            }
        };

        let options = PageOptions::new()
            .with_main_class("movie-review-post-page")
            .use_date_as_title()
            .with_image(&self.movie.poster);

        let page = Page::new(source.slug().clone(), None, None)
            .with_date(*source.date())
            .with_tags(source.tags().clone());

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&source.slug(), &rendered, Some(*source.date()))
    }
}
