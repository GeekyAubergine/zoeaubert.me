use crate::domain::models::data::Data;
use crate::domain::models::now_text::NowText;
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

pub fn render_now_page<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderNowPageTask {
        now_text: &data.now_text,
    });
}

struct RenderNowPageTask<'l> {
    now_text: &'l NowText,
}

impl<'l> RenderTask for RenderNowPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let now_text = self.now_text;

        let page = Page::new(Slug::new("/now"), Some("Now".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            article {
                (md(&now_text.now_text, MarkdownMediaOption::WithMedia))
            }
        };

        let options = PageOptions::new().with_main_class("now-page");

        let rended = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rended, None)
    }
}
