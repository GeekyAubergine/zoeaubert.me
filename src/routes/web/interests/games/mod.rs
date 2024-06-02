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
    domain::{games::games_models::Game, models::{image::Image, page::Page}},
    infrastructure::app_state::AppState,
};

use crate::utils::{FormatDate, FormatNumber};

const RECENT_GAMES_COUNT: usize = 3;
const HEADER_IMAGE_WIDTH: u32 = 414;
const HEADER_IMAGE_HEIGHT: u32 = 193;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/:id", get(game_page))
}

#[derive(Template)]
#[template(path = "interests/games/index.html")]
pub struct IndexTemplate {
    page: Page,
    games_by_recently_played: Vec<Game>,
    games_by_most_played: Vec<Game>,
    total_games: usize,
    total_playtime: f32,
}

async fn index(State(state): State<AppState>) -> IndexTemplate {
    let page = Page::new(
        state.site(),
        "/interests/games",
        Some("Games"),
        Some("Games I own"),
        None,
        None,
        None,
        vec![],
    )
    .set_no_index();

    let games_by_recently_played = state
        .games_repo()
        .get_games_by_most_recently_played()
        .await
        .iter()
        .take(RECENT_GAMES_COUNT)
        .cloned()
        .collect::<Vec<Game>>();

    let games_by_most_played = state.games_repo().get_games_by_most_played().await;

    let total_games = games_by_most_played.len();

    let total_playtime = state.games_repo().get_total_play_time_hours().await;

    IndexTemplate {
        page,
        games_by_recently_played,
        games_by_most_played,
        total_games,
        total_playtime,
    }
}

#[derive(Template)]
#[template(path = "interests/games/game.html")]
pub struct GameTemplate {
    page: Page,
    game: Game,
}

async fn game_page(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<GameTemplate, (StatusCode, &'static str)> {
    let game = state
        .games_repo()
        .get_game(id)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Game not found"))?;

    let title = format!("{} Game Stats", game.name());

    let mut description = match game.achievements_count() {
        0 => format!("{}h playtime", game.playtime_hours().format(1, true),),
        _ => format!(
            "{}h playtime, {}/{} achievements",
            game.playtime_hours().format(1, true),
            game.achievements_unlocked_count(),
            game.achievements_count(),
        ),
    };

    let image = Image::new(
        game.header_image_url(),
        format!("{} steam header image", game.name()).as_str(),
        HEADER_IMAGE_WIDTH,
        HEADER_IMAGE_HEIGHT,
        None,
        None,
        None,
        None,
    );

    let page = Page::new(
        state.site(),
        &format!("/interests/games/{}", game.id()),
        Some(title.as_str()),
        Some(description.as_str()),
        Some(image),
        None,
        None,
        vec![],
    )
    .set_no_index();

    Ok(GameTemplate { page, game })
}
