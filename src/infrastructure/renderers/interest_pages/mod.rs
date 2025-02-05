use book_pages_renderer::render_book_pages;
use game_pages_renderer::render_games_pages;
use interest_pages_renderer::render_interests_list_page;
use league_pages_renderer::render_league_pages;
use lego_pages_renderer::render_lego_page;
use movie_pages_renderer::render_movie_pages;
use tokio::try_join;
use tv_pages_show_renderer::render_tv_show_pages;

use crate::domain::state::State;
use crate::prelude::*;

pub mod book_pages_renderer;
pub mod game_pages_renderer;
pub mod interest_pages_renderer;
pub mod league_pages_renderer;
pub mod lego_pages_renderer;
pub mod movie_pages_renderer;
pub mod tv_pages_show_renderer;

pub async fn render_interest_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_interests_list_page(state),
        render_lego_page(state),
        render_games_pages(state),
        render_movie_pages(state),
        render_tv_show_pages(state),
        render_league_pages(state),
        render_book_pages(state)
    )?;

    Ok(())
}
