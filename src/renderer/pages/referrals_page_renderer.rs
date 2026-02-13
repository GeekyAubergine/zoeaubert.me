use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use hypertext::prelude::*;

use crate::renderer::RendererContext;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_referrals_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/save"), Some("Referrals".to_string()), None);
    let slug = page.slug.clone();

    let content = maud! {
        article {
            ul {
               @for referral in &context.data.referrals.referrals {
                   li {
                    h2 {
                        (referral.name)
                        a href=(referral.url.as_str()) {
                            p { (referral.url.as_str()) }
                        }
                    }
                    p { (referral.description) }
                   }
               }
            }
        }
    };

    let options = PageOptions::new().with_main_class("referrals-page");

    let render = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &render, None)
}
