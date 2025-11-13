use std::collections::HashMap;

use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::tag::{self, render_tags};
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_tags_pages(context: &RendererContext) -> Result<()> {
    let events = context.data.timeline_events.all_by_date();

    let mut events_by_tags: HashMap<&Tag, Vec<&TimelineEvent>> = HashMap::new();

    for event in events {
        for tag in event.tags() {
            events_by_tags
                .entry(tag)
                .or_insert_with(Vec::new)
                .push(event);
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
        let slug = page.slug.clone();

        let paginated = paginate(&events, PAGINATION_SIZE);

        for paginator_page in paginated {
            let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

            let slug = page.slug.clone();

            let content = render_timline_events_list(paginator_page.data);

            let options = PageOptions::new().with_main_class("tag-posts-page");

            let renderer = render_page(&page, &options, &content, None);

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

    let renderer = render_page(&page, &options, &content, None);

    context.renderer.render_page(&slug, &renderer, None)?;

    Ok(())
}
