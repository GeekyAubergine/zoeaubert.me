use crate::domain::state::State;
use crate::prelude::*;

use album_and_album_photo_pages_renderer::render_albums_and_album_photo_pages;
use photo_pages_renderer::render_photos_page;
use tokio::try_join;

pub mod album_and_album_photo_pages_renderer;
pub mod photo_pages_renderer;

pub async fn render_albums_and_photo_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_albums_and_album_photo_pages(state),
        render_photos_page(state),
    )?;

    Ok(())
}
