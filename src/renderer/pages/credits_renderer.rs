use crate::domain::models::credits::Credits;
use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RenderTasks;
use crate::renderer::RenderTask;
use hypertext::prelude::*;

use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_credits_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderCreditsPageTask {
        credits: &data.credits,
    });
}

struct RenderCreditsPageTask<'l> {
    credits: &'l Credits,
}

impl<'l> RenderTask for RenderCreditsPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(
            Slug::new("/credits"),
            Some("Credits & Attributions".to_string()),
            None,
        );
        let slug = page.slug.clone();

        let content = maud! {
            article {
                ul {
                    @for credit in self.credits {
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

        renderer.render_page(&slug, &render, None)
    }
}
