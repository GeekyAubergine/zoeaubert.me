use hypertext::prelude::*;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_timeline_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter(|event| match event {
            TimelineEvent::Post(_) => true,
            TimelineEvent::Review(_) => true,
            TimelineEvent::GameAchievementUnlock(_) => false,
            TimelineEvent::Album(_) => true,
            TimelineEvent::AlbumPhoto { .. } => false,
        })
        .collect::<Vec<&TimelineEvent>>();

    let paginated = paginate(&posts, PAGINATION_SIZE);

    let page = Page::new(Slug::new("/timeline"), Some("Timeline".to_string()), None);
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page);

        let slug = page.slug.clone();

        let content = render_timline_events_list(paginator_page.data);

        let options = PageOptions::new().with_main_class("timeline-page");

        let renderer = render_page(&page, &options, &content, maud! {});

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
