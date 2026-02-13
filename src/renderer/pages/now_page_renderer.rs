use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use hypertext::prelude::*;

use crate::renderer::RendererContext;
use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::md::md;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_now_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/now"), Some("Now".to_string()), None);
    let slug = page.slug.clone();

    let content = maud! {
        article {
            (md(&context.data.now_text.now_text, MarkdownMediaOption::WithMedia))
        }
    };

    let options = PageOptions::new().with_main_class("now-page");

    let render = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &render, None)
}
