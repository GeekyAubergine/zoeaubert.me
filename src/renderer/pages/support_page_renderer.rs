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

const CONTENT: &str = r#"<p>
    Like what I do? Consider
    <a href="https://buymeacoffee.com/zoeaubert">supporting me</a>
</p>"#;

pub fn render_support_page<'d>(tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderSupportPageTask);
}

struct RenderSupportPageTask;

impl RenderTask for RenderSupportPageTask {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/support"), Some("Support".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            article {
                (md(&CONTENT, MarkdownMediaOption::WithMedia))
            }
        };

        let options = PageOptions::new().with_main_class("support-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
