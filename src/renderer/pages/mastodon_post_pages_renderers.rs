use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::models::page::Page;
use crate::prelude::*;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTasks, RenderTask};

pub fn render_mastodon_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events
        .mastodon_posts_by_date()
        .for_each(|post| {
            tasks.add(RenderMastodonPostPageTask { post });
        });
}

struct RenderMastodonPostPageTask<'p> {
    post: &'p MastodonPost,
}

impl<'p> RenderTask for RenderMastodonPostPageTask<'p> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let post = self.post;

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

        renderer.render_page(&post.slug(), &rendered, Some(*post.created_at()))
    }
}
