use crate::{
    domain::{
        models::{page::Page, slug::Slug}, services::PageRenderingService, state::State
    },
    prelude::*,
};

use askama::Template;
use faq_page_renderer::render_faq_page;
use now_page_renderer::render_now_page;
use save_page_renderer::render_save_page;
use tokio::try_join;

pub mod faq_page_renderer;
pub mod now_page_renderer;
pub mod save_page_renderer;

pub async fn render_basic_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_save_page(state),
        render_faq_page(state),
        render_now_page(state),
        render_404_page(state),
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct FourOFourTemplate {
    page: Page,
}

async fn render_404_page(state: &impl State) -> Result<()> {
    let page = Page::new(Slug::new("/404"), None, None);

    state
        .page_rendering_service()
        .add_page(state, page.slug.clone(), FourOFourTemplate { page }, None)
        .await
}
