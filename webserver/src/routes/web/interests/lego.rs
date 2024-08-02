use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

use crate::{
    build_data,
    domain::models::{
        game::Game,
        game_achievement::{GameAchievement, GameAchievements},
        lego::{LegoMinifig, LegoSet},
        media::image::Image,
        page::Page,
    },
    infrastructure::app_state::AppState,
    TemplateResult,
};

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::markdown::FormatMarkdown;
pub use crate::infrastructure::services::number::FormatNumber;

const RECENT_GAMES_COUNT: usize = 6;
const HEADER_IMAGE_WIDTH: u32 = 414;
const HEADER_IMAGE_HEIGHT: u32 = 193;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list))
}

#[derive(Template)]
#[template(path = "interests/lego_list.html")]
pub struct LegoListTemplate {
    page: Page,
    total_sets: u32,
    total_pieces: u32,
    sets: Vec<LegoSet>,
    total_minifigs: u32,
    minifigs: Vec<LegoMinifig>,
}

async fn list(State(state): State<AppState>) -> TemplateResult<LegoListTemplate> {
    let page = Page::new(
        state.site(),
        "/interests/lego",
        Some("Lego"),
        Some("My Lgo Collection"),
    );

    let sets = state.lego_set_repo().find_all_sort_by_most_pieces().await?;

    let total_pieces = state.lego_set_repo().find_total_pieces().await?;

    let total_sets = state.lego_set_repo().find_total_owned().await?;

    let total_minifigs = state.lego_minifigs_repo().find_total_owned().await?;

    let minifigs = state
        .lego_minifigs_repo()
        .find_all_sorted_by_category_and_name()
        .await?;

    Ok(LegoListTemplate {
        page,
        total_pieces,
        total_sets: sets.len() as u32,
        sets,
        total_minifigs,
        minifigs,
    })
}
