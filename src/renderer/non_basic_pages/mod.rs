use album_pages_renderer::render_album_pages;
use feed_page_renderers::render_feed_files;
use home_page_renderer::render_home_page;
use photo_pages_renderer::render_photos_page;
use project_pages_renderers::render_project_pages;
use tokio::try_join;

use crate::domain::models::data::Data;
use crate::prelude::*;

use super::RendererContext;

pub mod album_pages_renderer;
pub mod feed_page_renderers;
pub mod home_page_renderer;
pub mod photo_pages_renderer;
pub mod project_pages_renderers;

pub async fn render_non_basic_pages(
    context: &RendererContext,
) -> Result<()> {
    try_join!(
        render_album_pages(context),
        render_feed_files(context),
        render_home_page(context),
        render_photos_page(context),
        render_project_pages(context),
    )?;

    Ok(())
}
