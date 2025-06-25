use std::path::Path;
use std::sync::Arc;

use basic_pages::render_basic_pages;
use interest_pages::render_interest_pages;
use non_basic_pages::render_non_basic_pages;
use posts::render_post_pages;
use tokio::try_join;

use crate::domain::models::data::Data;
use crate::prelude::*;
use crate::services::page_renderer::PageRenderer;
use tracing::debug;

use crate::domain::models::page::Page;
use crate::error::{FileSystemError, TemplateError};

pub mod basic_pages;
pub mod formatters;
pub mod interest_pages;
pub mod non_basic_pages;
pub mod posts;

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
    try_join!(
        render_basic_pages(context),
        render_interest_pages(context),
        render_non_basic_pages(context),
        render_post_pages(context),
    )?;

    Ok(())
}
