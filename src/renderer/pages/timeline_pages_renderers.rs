use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::renderer::RenderTasks;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::timeline_events_list::RenderTimelineEventsListTask;
use crate::utils::paginator::Paginator;

const PAGINATION_SIZE: usize = 25;

pub fn render_timeline_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    let page = Page::new(Slug::new("/timeline"), Some("Timeline".to_string()), None);
    let options = PageOptions::new().with_main_class("timeline-page");

    data.timeline_events
        .all_by_date()
        .iter()
        .filter(|event| match event {
            TimelineEvent::Post(_) => true,
            TimelineEvent::Review(_) => true,
            TimelineEvent::GameAchievementUnlock(_) => false,
            TimelineEvent::Album(_) => true,
            TimelineEvent::AlbumPhoto { .. } => false,
        })
        .paginate(PAGINATION_SIZE)
        .for_each(|paginator_page| {
            tasks.add(RenderTimelineEventsListTask::new(
                paginator_page,
                page.clone(),
                options.clone(),
            ))
        });
}
