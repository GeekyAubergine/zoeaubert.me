use crate::{
    domain::{
        models::{page::Page, slug::Slug},
    },
    prelude::*,
};

use askama::Template;
use faq_page_renderer::render_faq_page;
use now_page_renderer::render_now_page;
use save_page_renderer::render_save_page;
use support_page_renderer::render_support_page;
use tokio::try_join;

use super::RendererContext;

pub mod faq_page_renderer;
pub mod now_page_renderer;
pub mod save_page_renderer;
pub mod support_page_renderer;

pub async fn render_basic_pages(context: &RendererContext) -> Result<()> {
    try_join!(
        render_save_page(context),
        render_faq_page(context),
        render_now_page(context),
        render_support_page(context),
        render_404_page(context),
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct FourOFourTemplate {
    page: Page,
}

async fn render_404_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/404"), None, None);

    let template = FourOFourTemplate { page };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
