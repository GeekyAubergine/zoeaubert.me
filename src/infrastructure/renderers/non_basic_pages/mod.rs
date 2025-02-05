use album_pages_renderer::render_album_pages;
use feed_page_renderers::render_feed_files;
use home_page_renderer::render_home_page;
use photo_pages_renderer::render_photos_page;
use project_pages_renderers::render_project_pages;
use tokio::try_join;

use crate::domain::state::State;
use crate::prelude::*;


pub mod album_pages_renderer;
pub mod feed_page_renderers;
pub mod home_page_renderer;
pub mod photo_pages_renderer;
pub mod project_pages_renderers;

pub async fn render_non_basic_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_album_pages(state),
        render_feed_files(state),
        render_home_page(state),
        render_photos_page(state),
        render_project_pages(state),
    )?;

    Ok(())
}
