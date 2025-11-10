use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::book::Book;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::page::Page;
use crate::domain::models::review::book_review::BookReview;
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
use crate::renderer::partials::utils::link;
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
            TimelineEvent::BookReview { review, book } => Some((review, book)),
            _ => None,
        })
        .collect::<Vec<(&BookReview, &Book)>>();

    for (review, book) in posts {
        render_book_review_page(context, review, book)?;
    }

    Ok(())
}

pub fn render_book_review_page(
    context: &RendererContext,
    review: &BookReview,
    book: &Book,
) -> Result<()> {
    let content = maud! {
        article {
            (md(&review.source.content(), md::MarkdownMediaOption::NoMedia))
        }
    };

    let options = PageOptions::new()
        .with_main_class("book-review-post-page")
        .use_date_as_title()
        .with_image(&book.cover);

    let page = Page::new(review.source.slug().clone(), None, None)
        .with_date(review.source.date().clone())
        .with_tags(review.source.tags().clone());

    let rendered = render_page(&page, &options, &content, None);

    context.renderer.render_page(
        &review.source.slug(),
        &rendered,
        Some(review.source.date().clone()),
    )
}
