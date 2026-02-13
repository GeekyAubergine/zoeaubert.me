use hypertext::prelude::*;

use crate::domain::models::page::Page;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::review::tv_show_review::TvShowReview;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::tv_show::TvShow;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};

// TODO Clicking on cover image should link you to tmdb page

pub fn render_tv_review_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::TvShowReview {
                review,
                tv_show,
                source,
            }) => Some((review, tv_show, source)),
            _ => None,
        })
        .collect::<Vec<(&TvShowReview, &TvShow, &ReviewSource)>>();

    for (_, tv, source) in posts {
        render_tv_review_page(context, tv, source)?;
    }

    Ok(())
}

pub fn render_tv_review_page(
    context: &RendererContext,
    tv_show: &TvShow,
    source: &ReviewSource,
) -> Result<()> {
    let content = maud! {
        article {
            (md(&source.content(), md::MarkdownMediaOption::NoMedia))
        }
    };

    let options = PageOptions::new()
        .with_main_class("tv-show-review-post-page")
        .use_date_as_title()
        .with_image(&tv_show.poster);

    let page = Page::new(source.slug().clone(), None, None)
        .with_date(*source.date())
        .with_tags(source.tags().clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context
        .renderer
        .render_page(&source.slug(), &rendered, Some(*source.date()))
}
