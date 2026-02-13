use std::collections::HashMap;

use hypertext::prelude::*;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_tags_pages(context: &RendererContext) -> Result<()> {
    let events = context.data.timeline_events.all_by_date();

    let mut events_by_tags: HashMap<&Tag, Vec<&TimelineEvent>> = HashMap::new();

    for event in events {
        if let Some(tags) = event.tags() {
            for tag in tags {
                events_by_tags
                    .entry(tag)
                    .or_insert_with(Vec::new)
                    .push(event);
            }
        }
    }

    let mut tag_groups = events_by_tags
        .into_iter()
        .map(|(tag, posts)| (tag, posts))
        .collect::<Vec<(&Tag, Vec<&TimelineEvent>)>>();

    tag_groups.sort_by(|a, b| a.0.tag.cmp(&b.0.tag));

    for (tag, events) in tag_groups.iter() {
        let page = Page::new(
            Slug::new(&format!("/tags/{}", tag.slug())),
            Some(format!("{} Posts", tag.title())),
            Some(format!("#{} posts", tag.title())),
        );
        let paginated = paginate(&events, PAGINATION_SIZE);

        for paginator_page in paginated {
            let page = Page::from_page_and_pagination_page(&page, &paginator_page);

            let slug = page.slug.clone();

            let content = render_timline_events_list(paginator_page.data);

            let options = PageOptions::new().with_main_class("tag-posts-page");

            let renderer = render_page(&page, &options, &content, maud! {});

            context.renderer.render_page(&slug, &renderer, None)?;
        }
    }

    let page = Page::new(Slug::new("/tags"), Some("Tags".to_string()), None);
    let slug = page.slug.clone();

    let content = maud! {
        ul class="tags-grid" {
            @for (tag, posts) in &tag_groups {
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

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)?;

    Ok(())
}
