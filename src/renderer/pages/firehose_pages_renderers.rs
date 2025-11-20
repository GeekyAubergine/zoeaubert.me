use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, PageWidth, render_page};
use crate::renderer::partials::tag::render_tags;
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_firehose_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter(|event| match event {
            TimelineEvent::Post(_) => true,
            TimelineEvent::Review(_) => true,
        })
        .collect::<Vec<&TimelineEvent>>();

    let paginated = paginate(&posts, PAGINATION_SIZE);

    let page = Page::new(Slug::new("/firehose"), Some("Firehose".to_string()), None);
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let slug = page.slug.clone();

        let content = render_timline_events_list(paginator_page.data);

        let options = PageOptions::new().with_main_class("firehose-page");

        let renderer = render_page(&page, &options, &content, maud! {});

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
