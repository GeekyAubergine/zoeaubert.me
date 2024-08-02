use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use uuid::Uuid;

use crate::{
    build_data,
    domain::models::{
        game::Game,
        game_achievement::{GameAchievement, GameAchievements},
        media::image::Image,
        page::{Page, PageImage},
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

const HEADER_IMAGE_UUID_SEED: u128 = 0x238f888c8d8b8b773;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/:id", get(game_page))
        .route("/:id/", get(game_page))
}

#[derive(Template)]
#[template(path = "interests/games/games_list.html")]
pub struct IndexTemplate {
    page: Page,
    games_by_recently_played: Vec<Game>,
    games_by_most_played: Vec<Game>,
    total_games: usize,
    total_playtime: f32,
}

async fn index(State(state): State<AppState>) -> TemplateResult<IndexTemplate> {
    let page = Page::new(
        state.site(),
        "/interests/games",
        Some("Games"),
        Some("Games I own"),
    );

    let games_by_recently_played = state
        .games_repo()
        .get_games_by_most_recently_played()
        .await?
        .iter()
        .take(RECENT_GAMES_COUNT)
        .cloned()
        .collect::<Vec<Game>>();

    let games_by_most_played = state.games_repo().get_games_by_most_played().await?;

    let total_games = games_by_most_played.len();

    let total_playtime = state.games_repo().get_total_play_time_hours().await?;

    Ok(IndexTemplate {
        page,
        games_by_recently_played,
        games_by_most_played,
        total_games,
        total_playtime,
    })
}

#[derive(Template)]
#[template(path = "interests/games/game.html")]
pub struct GameTemplate {
    page: Page,
    game: Game,
    achievements: GameAchievements,
}

async fn game_page(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> TemplateResult<GameTemplate> {
    let game = state
        .games_repo()
        .find_by_id(id)
            .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Game not found"))?
        .ok_or((StatusCode::NOT_FOUND, "Game not found"))?;

    let achievements = state.game_achievements_repo().find_by_game_id(id).await?;

    let title = format!("{} Game Stats", game.name());

    let mut description = match achievements.achievements_count() {
        0 => format!("{}h playtime", game.playtime_hours().format(1, true),),
        _ => format!(
            "{}h playtime, {}/{} achievements",
            game.playtime_hours().format(1, true),
            achievements.achievements_unlocked_count(),
            achievements.achievements_count(),
        ),
    };

    let image = PageImage::new(
        game.header_image_url(),
        format!("{} Steam header image", game.name()).as_str(),
        HEADER_IMAGE_WIDTH,
        HEADER_IMAGE_HEIGHT,
    );

    let page = Page::new(
        state.site(),
        &format!("/interests/games/{}", game.id()),
        Some(title.as_str()),
        Some(description.as_str()),
    )
    .with_image(image);

    Ok(GameTemplate {
        page,
        game,
        achievements,
    })
}
