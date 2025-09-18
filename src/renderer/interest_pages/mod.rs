use book_pages_renderer::render_book_pages;
use game_pages_renderer::render_games_pages;
use interest_pages_renderer::render_interests_list_page;
use lego_pages_renderer::render_lego_page;
use movie_pages_renderer::render_movie_pages;
use tokio::try_join;
use tv_pages_show_renderer::render_tv_show_pages;

use crate::prelude::*;

use super::RendererContext;

pub mod book_pages_renderer;
pub mod game_pages_renderer;
pub mod interest_pages_renderer;
pub mod lego_pages_renderer;
pub mod movie_pages_renderer;
pub mod tv_pages_show_renderer;

pub async fn render_interest_pages(context: &RendererContext) -> Result<()> {
    try_join!(
        render_interests_list_page(context),
        render_lego_page(context),
        render_games_pages(context),
        render_movie_pages(context),
        render_tv_show_pages(context),
        render_book_pages(context)
    )?;

    Ok(())
}
