use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::review::tv_show_review::TvShowReview;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::tv_show::TvShow;
use crate::prelude::*;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTasks, RenderTask};

// TODO Clicking on cover image should link you to tmdb page

pub fn render_tv_review_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::TvShowReview {
                tv_show, source, ..
            }) => Some((tv_show, source)),
            _ => None,
        })
        .for_each(|(tv_show, source)| tasks.add(RenderTvShowReviewPageTask { tv_show, source }));
}

struct RenderTvShowReviewPageTask<'l> {
    tv_show: &'l TvShow,
    source: &'l ReviewSource,
}

impl<'l> RenderTask for RenderTvShowReviewPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let source = self.source;

        let content = maud! {
            article {
                (md(&source.content(), md::MarkdownMediaOption::NoMedia))
            }
        };

        let options = PageOptions::new()
            .with_main_class("tv-show-review-post-page")
            .use_date_as_title()
            .with_image(&self.tv_show.poster);

        let page = Page::new(source.slug().clone(), None, None)
            .with_date(*source.date())
            .with_tags(source.tags().clone());

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&source.slug(), &rendered, Some(*source.date()))
    }
}
