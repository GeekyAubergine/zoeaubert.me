use hypertext::prelude::*;

use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::page::Page;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::page::{PageOptions, render_page};

pub fn render_mastodon_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(TimelineEventPost::MastodonPost(post)) => Some(post),
            _ => None,
        })
        .collect::<Vec<&Box<MastodonPost>>>();

    for post in posts {
        render_mastodon_post_page(context, post)?;
    }

    Ok(())
}

pub fn render_mastodon_post_page(context: &RendererContext, post: &MastodonPost) -> Result<()> {
    let content = maud! {
        article {
            (md(&post.content(), md::MarkdownMediaOption::NoMedia))
            (render_media_grid(post.media(), &MediaGripOptions::for_post()))
            p class="original-link" {
                ("See Original: ")
                a href=(post.original_uri().as_str()) class="link" target="_blank" rel="me" {
                    (post.original_uri().as_str())
                }
            }
        }
    };

    let options = PageOptions::new()
        .with_main_class("mastodon-post-page")
        .use_date_as_title();

    let page = Page::new(post.slug().clone(), None, None)
        .with_date(*post.created_at())
        .with_tags(post.tags().clone());

    let rendered = render_page(&page, &options, &content, maud! {});

    context
        .renderer
        .render_page(&post.slug(), &rendered, Some(*post.created_at()))
}
