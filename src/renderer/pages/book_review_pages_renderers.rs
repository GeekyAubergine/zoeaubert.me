use hypertext::prelude::*;

use crate::domain::models::book::Book;
use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::review::review_source::ReviewSource;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::prelude::*;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTask, RenderTasks};
use crate::services::page_renderer::PageRenderer;

// TODO Clicking on cover image should link you to open library page

pub fn render_book_review_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Review(TimelineEventReview::BookReview { book, source, .. }) => {
                Some((book, source))
            }
            _ => None,
        })
        .for_each(|(book, source)| tasks.add(RenderBookReviewPageTask { book, source }));
}

struct RenderBookReviewPageTask<'l> {
    book: &'l Book,
    source: &'l ReviewSource,
}

impl<'l> RenderTask for RenderBookReviewPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let source = self.source;

        let content = maud! {
            article {
                (md(&source.content(), md::MarkdownMediaOption::NoMedia))
            }
        };

        let options = PageOptions::new()
            .with_main_class("book-review-post-page")
            .use_date_as_title()
            .with_image(&self.book.cover);

        let page = Page::new(source.slug().clone(), None, None)
            .with_date(*source.date())
            .with_tags(source.tags().clone());

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&source.slug(), &rendered, Some(*source.date()))
    }
}
