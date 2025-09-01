use std::path::Path;
use std::sync::Arc;

use askama::Template;
// use basic_pages::render_basic_pages;
// use interest_pages::render_interest_pages;
// use non_basic_pages::render_non_basic_pages;
// use posts::render_post_pages;
use tokio::try_join;

use crate::domain::models::data::Data;
use crate::prelude::*;
use crate::renderer::pages::home_page_renderer::render_home_page;
use crate::renderer::pages::render_blog_page;
use crate::services::page_renderer::PageRenderer;
use tracing::debug;

use crate::domain::models::page::Page;
use crate::error::{FileSystemError, TemplateError};

// pub mod basic_pages;
pub mod formatters;

pub mod partials;
pub mod pages;

pub struct RendererContext {
    pub data: Data,
    pub renderer: PageRenderer,
}

pub async fn new_rendering_context_from_data(data: Data) -> Result<RendererContext> {
    Ok(RendererContext {
        data,
        renderer: PageRenderer::new(),
    })
}

pub async fn render_pages(context: &RendererContext) -> Result<()> {
    // try_join!(
    //     render_basic_pages(context),
    //     render_interest_pages(context),
    //     render_non_basic_pages(context),
    //     render_post_pages(context),
    // )?;

    render_home_page(context).await?;
    render_blog_page(context).await?;

    Ok(())
}

pub type TemplateRenderResult = Result<String>;

pub fn render_template<T: Template>(template: T) -> TemplateRenderResult {
    template.render().map_err(TemplateError::render_error)
}
