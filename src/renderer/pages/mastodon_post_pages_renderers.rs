use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{render_media_grid, MediaGripOptions};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

pub fn render_mastodon_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::MastodonPost(post) => Some(post),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<&MastodonPost>>();

    for post in posts {
        render_mastodon_post_page(context, post)?;
    }

    Ok(())
}

pub fn render_mastodon_post_page(context: &RendererContext, post: &MastodonPost) -> Result<()> {
    let content = maud! {
        article {
            (md(&post.content(), md::MarkdownMediaOption::NoMedia))
            (render_media_grid(&post.media(), &MediaGripOptions::for_post()))
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
        .with_date(post.created_at().clone())
        .with_tags(post.tags().clone());

    let rendered = render_page(&page, &options, &content, None);

    context
        .renderer
        .render_page(&post.slug(), &rendered, Some(post.created_at().clone()))
}
