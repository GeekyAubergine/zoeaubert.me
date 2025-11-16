use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::book::Book;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::page::Page;
use crate::domain::models::review::book_review::BookReview;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{render_media_grid, MediaGripOptions};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

// TODO Clicking on cover image should link you to open library page

pub fn render_book_review_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::BookReview { review, book, source } => Some((review, book, source)),
            _ => None,
        })
        .collect::<Vec<(&BookReview, &Book, &ReviewSource)>>();

    for (review, book, source) in posts {
        render_book_review_page(context, review, book, source)?;
    }

    Ok(())
}

pub fn render_book_review_page(
    context: &RendererContext,
    review: &BookReview,
    book: &Book,
    source: &ReviewSource,
) -> Result<()> {
    let content = maud! {
        article {
            (md(&source.content(), md::MarkdownMediaOption::NoMedia))
        }
    };

    let options = PageOptions::new()
        .with_main_class("book-review-post-page")
        .use_date_as_title()
        .with_image(&book.cover);

    let page = Page::new(source.slug().clone(), None, None)
        .with_date(source.date().clone())
        .with_tags(source.tags().clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(
        &source.slug(),
        &rendered,
        Some(source.date().clone()),
    )
}
