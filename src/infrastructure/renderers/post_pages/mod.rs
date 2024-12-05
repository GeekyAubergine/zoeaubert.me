use blog_pages_renderer::render_blog_pages;
use mastodon_post_pages_renderer::render_mastodon_post_pages;
use micro_post_pages_renderer::render_micro_post_pages;
use tags_pages_renderer::render_tags_pages;
use timeline_pages_renderer::render_timeline_page;
use tokio::try_join;
use years_pages_renderer::render_years_pages;

use crate::domain::state::State;
use crate::prelude::*;

pub mod blog_pages_renderer;
pub mod mastodon_post_pages_renderer;
pub mod micro_post_pages_renderer;
pub mod tags_pages_renderer;
pub mod timeline_pages_renderer;
pub mod years_pages_renderer;

pub async fn render_post_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_blog_pages(state),
        render_micro_post_pages(state),
        render_mastodon_post_pages(state),
        render_timeline_page(state),
        render_tags_pages(state),
        render_years_pages(state),
    )?;

    Ok(())
}
