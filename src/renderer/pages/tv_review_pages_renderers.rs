use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::book::Book;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::movie::Movie;
use crate::domain::models::page::Page;
use crate::domain::models::review::book_review::BookReview;
use crate::domain::models::review::movie_review::MovieReview;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::review::tv_show_review::TvShowReview;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::{
    TimelineEvent, TimelineEventPost, TimelineEventReview,
};
use crate::domain::models::tv_show::TvShow;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::page::{PageOptions, PageWidth, render_page};
use crate::renderer::partials::tag::render_tags;
use crate::utils::paginator::paginate;

// TODO Clicking on cover image should link you to tmdb page

pub fn render_tv_review_pages(context: &RendererContext) -> Result<()> {
    let posts = context
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
                } => Some((review, tv_show, source)),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<(&TvShowReview, &TvShow, &ReviewSource)>>();

    for (review, tv, source) in posts {
        render_tv_review_page(context, review, tv, source)?;
    }

    Ok(())
}

pub fn render_tv_review_page(
    context: &RendererContext,
    review: &TvShowReview,
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
        .with_date(source.date().clone())
        .with_tags(source.tags().clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context
        .renderer
        .render_page(&source.slug(), &rendered, Some(source.date().clone()))
}
