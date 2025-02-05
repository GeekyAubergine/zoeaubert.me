use blog_post_list_renderer::render_blog_list_page;
use tags_pages_renderer::render_tags_pages;
use timeline_pages_renderer::render_timeline_pages;
use tokio::try_join;
use years_pages_renderer::render_years_pages;

use crate::domain::state::State;
use crate::prelude::*;

pub mod blog_post_list_renderer;
pub mod omni_post_pages_renderer;
pub mod timeline_pages_renderer;
pub mod years_pages_renderer;
pub mod tags_pages_renderer;

pub async fn render_omni_post_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_blog_list_page(state),
        render_years_pages(state),
        render_timeline_pages(state),
        omni_post_pages_renderer::render_omni_post_pages(state),
        render_tags_pages(state,)
    )?;

    Ok(())
}
