use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use hypertext::prelude::*;

use crate::renderer::RendererContext;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_credits_pages(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/credits"),
        Some("Credits & Attributions".to_string()),
        None,
    );
    let slug = page.slug.clone();

    let content = maud! {
        article {
            ul {
                @for credit in &context.data.credits {
                    li {
                        a href=(credit.url.as_str()) {
                            h2 { (credit.name ) }
                        }
                        p { (credit.text) }
                    }
                }
            }
        }
    };

    let options = PageOptions::new().with_main_class("credits-page");

    let render = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &render, None)
}
