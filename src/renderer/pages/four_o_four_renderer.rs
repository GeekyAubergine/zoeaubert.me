use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RenderTask;
use crate::renderer::RenderTasks;
use hypertext::prelude::*;

use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_404_page<'d>(tasks: &mut RenderTasks<'d>) {
    tasks.add(Render404PageTask);
}

struct Render404PageTask;

impl RenderTask for Render404PageTask {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(Slug::new("/404"), Some("Page Not Found".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            p class="text-center mx-auto" { "Sorry, the page you’re looking for could not be found" }
        };

        let options = PageOptions::new().with_main_class("feeds-page");

        let render = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &render, None)
    }
}
