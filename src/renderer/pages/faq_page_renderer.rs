use crate::domain::models::data::Data;
use crate::domain::models::faq::Faq;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RenderTasks;
use crate::renderer::RenderTask;
use crate::services::page_renderer::PageRenderer;
use hypertext::prelude::*;

use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::md::md;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_faq_page<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderFaqPageTask { faq: &data.faq });
}

struct RenderFaqPageTask<'l> {
    faq: &'l Faq,
}

impl<'l> RenderTask for RenderFaqPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/faq"), Some("FAQ".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            article {
                (md(&self.faq.faq, MarkdownMediaOption::WithMedia))
            }
        };

        let options = PageOptions::new().with_main_class("faq-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
