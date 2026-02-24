use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::micro_post::MicroPost;
use crate::domain::models::page::Page;
use crate::prelude::*;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::media::{MediaGripOptions, render_media_grid};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTasks, RenderTask};
use crate::services::page_renderer::PageRenderer;

pub fn render_micro_post_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events.micro_posts_by_date().for_each(|post| {
        tasks.add(RenderMicroPostTask { post });
    })
}

struct RenderMicroPostTask<'p> {
    post: &'p MicroPost,
}

impl<'p> RenderTask for RenderMicroPostTask<'p> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let post = self.post;

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

        renderer.render_page(&post.slug, &rendered, Some(post.date))
    }
}
