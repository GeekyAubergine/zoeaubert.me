use hypertext::prelude::*;

use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::page::Page;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::page::{PageOptions, render_page};

pub fn render_micro_post_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(TimelineEventPost::MicroPost(post)) => Some(post),
            _ => None,
        })
        .collect::<Vec<&Box<MicroPost>>>();

    for post in posts {
        render_micro_post_page(context, post)?;
    }

    Ok(())
}

pub fn render_micro_post_page(context: &RendererContext, post: &MicroPost) -> Result<()> {
    let content = maud! {
        article {
            (md(&post.content, md::MarkdownMediaOption::NoMedia))
            (render_media_grid(post.media(), &MediaGripOptions::for_post()))
        }
    };

    let options = PageOptions::new()
        .with_main_class("micro-post-page")
        .use_date_as_title();

    let page = Page::new(post.slug.clone(), None, None)
        .with_date(post.date)
        .with_tags(post.tags.clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context
        .renderer
        .render_page(&post.slug, &rendered, Some(post.date))
}
