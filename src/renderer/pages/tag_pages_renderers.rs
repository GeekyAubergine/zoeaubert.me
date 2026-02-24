use std::collections::HashMap;

use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::prelude::*;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::timeline_events_list::RenderTimelineEventsListTask;
use crate::renderer::{RenderTask, RenderTasks};
use crate::services::page_renderer::PageRenderer;
use crate::utils::paginator::Paginator;

const PAGINATION_SIZE: usize = 25;

pub fn render_tags_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    let events = data.timeline_events.all_by_date();

    let mut events_by_tags: HashMap<&Tag, Vec<&TimelineEvent>> = HashMap::new();

    for event in events {
        if let Some(tags) = event.tags() {
            for tag in tags {
                events_by_tags.entry(tag).or_default().push(event);
            }
        }
    }

    let mut tag_groups = events_by_tags
        .into_iter()
        .collect::<Vec<(&Tag, Vec<&TimelineEvent>)>>();

    tag_groups.sort_by_key(|(tag, _)| &tag.tag);

    tasks.add(RenderTagsListPageTask {
        tag_groups: tag_groups.clone(),
    });

    // Tag speicifc pages
    for (tag, events) in tag_groups.into_iter() {
        let page = Page::new(
            Slug::new(&format!("/tags/{}", tag.slug())),
            Some(format!("{} Posts", tag.title())),
            Some(format!("#{} posts", tag.title())),
        );

        let page_options = PageOptions::new().with_main_class("tag-posts-page");

        events
            .into_iter()
            .paginate(PAGINATION_SIZE)
            .for_each(|paginator_page| {
                tasks.add(RenderTimelineEventsListTask::new(
                    paginator_page,
                    page.clone(),
                    page_options.clone(),
                ))
            });
    }
}

struct RenderTagsListPageTask<'l> {
    tag_groups: Vec<(&'l Tag, Vec<&'l TimelineEvent>)>,
}

impl<'l> RenderTask for RenderTagsListPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/tags"), Some("Tags".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            ul class="tags-grid" {
                @for (tag, posts) in &self.tag_groups {
                    li {
                        a href=(format!("/tags/{}", &tag.slug())) {
                            p class="tag" { "#" (&tag.tag) }
                            p class="count" { (posts.len()) }
                        }
                    }
                }
            }
        };

        let options = PageOptions::new().with_main_class("tags-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
