use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RenderTasks;
use crate::renderer::RenderTask;
use crate::services::page_renderer::PageRenderer;
use hypertext::prelude::*;

use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_feeds_page<'d>(tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderFeedsPageTask);
}

struct RenderFeedsPageTask;

impl RenderTask for RenderFeedsPageTask {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/feeds"), Some("Feeds".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            article {
                ul {
                    li {
                        a href=("/feeds/blog-rss.xml") {
                            p { ("Blog Posts RSS") }
                        }
                    }
                }
            }
        };

        let options = PageOptions::new().with_main_class("feeds-page");

        let render = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &render, None)
    }
}
