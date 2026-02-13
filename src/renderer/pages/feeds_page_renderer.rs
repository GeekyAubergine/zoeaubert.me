use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use hypertext::prelude::*;

use crate::renderer::RendererContext;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_feeds_page(context: &RendererContext) -> Result<()> {
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

    context.renderer.render_page(&slug, &render, None)
}
