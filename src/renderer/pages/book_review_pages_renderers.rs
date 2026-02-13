use hypertext::prelude::*;

use crate::domain::models::book::Book;
use crate::domain::models::page::Page;
use crate::domain::models::review::book_review::BookReview;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};

// TODO Clicking on cover image should link you to open library page

pub fn render_book_review_pages(context: &RendererContext) -> Result<()> {
    let posts = context
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
                } => Some((review, book, source)),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<(&BookReview, &Book, &ReviewSource)>>();

    for (_, book, source) in posts {
        render_book_review_page(context, book, source)?;
    }

    Ok(())
}

pub fn render_book_review_page(
    context: &RendererContext,
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

    context
        .renderer
        .render_page(&source.slug(), &rendered, Some(source.date().clone()))
}
