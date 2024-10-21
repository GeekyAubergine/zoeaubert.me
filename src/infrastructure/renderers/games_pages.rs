use askama::Template;

use crate::domain::models::games::{GameAchievementLocked, GameAchievementUnlocked};
use crate::domain::models::site_config::PageImage;
use crate::domain::models::slug::Slug;
use crate::domain::models::{games::Game, page::Page};

use crate::domain::repositories::GameAchievementsRepo;
use crate::prelude::*;

use crate::domain::state::State;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::render_page_with_template;

const RECENTLY_PLAYED_GAMES_COUNT: usize = 6;
const HEADER_IMAGE_WIDTH: u32 = 414;
const HEADER_IMAGE_HEIGHT: u32 = 193;

#[derive(Template)]
#[template(path = "interests/games/games_list.html")]
pub struct IndexTemplate<'t> {
    page: &'t Page<'t>,
    games_by_recently_played: Vec<&'t Game>,
    games_by_most_played: Vec<Game>,
    total_games: usize,
    total_playtime: f32,
}

pub async fn render_games_list_page(state: &impl State, games: &[Game]) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games"),
        Some("Games"),
        Some("My Games"),
    );

    let mut games_by_most_played = games.to_vec();

    games_by_most_played.sort_by(|a, b| b.playtime.partial_cmp(&a.playtime).unwrap());

    let mut games_by_recently_played = games.to_vec();

    games_by_recently_played.sort_by(|a, b| b.last_played.partial_cmp(&a.last_played).unwrap());

    let games_by_recently_played = games_by_recently_played
        .iter()
        .take(RECENTLY_PLAYED_GAMES_COUNT)
        .collect::<Vec<_>>();

    let total_games = games.len();
    let total_playtime = games.iter().map(|g| g.playtime_hours()).sum::<f32>();

    let template = IndexTemplate {
        page: &page,
        games_by_recently_played,
        games_by_most_played,
        total_games,
        total_playtime,
    };

    render_page_with_template(state, &page, template).await
}

#[derive(Template)]
#[template(path = "interests/games/game.html")]
pub struct GameTemplate<'t> {
    page: &'t Page<'t>,
    game: &'t Game,
    unlocked_achievements: Vec<&'t GameAchievementUnlocked>,
    locked_achievements: Vec<&'t GameAchievementLocked>,
    total_achievements: usize,
}

pub async fn render_game_page(state: &impl State, game: &Game) -> Result<()> {
    let unlocked_achievements = state
        .game_achievements_repo()
        .find_all_unlocked_by_unlocked_date(game.id)
        .await?;

    let locked_achievements = state
        .game_achievements_repo()
        .find_all_locked_by_name(game.id)
        .await?;

    let total_achievements = unlocked_achievements.len() + locked_achievements.len();

    let title = format!("{} Game Stats", game.name);

    let description = match total_achievements {
        0 => format!("{}h playtime", game.playtime_hours().format(1, true),),
        _ => format!(
            "{}h playtime, {}/{} achievements",
            game.playtime_hours().format(1, true),
            unlocked_achievements.len(),
            total_achievements,
        ),
    };

    let image = PageImage::new(
        game.header_image.cdn_url().as_str(),
        format!("{} Steam header image", game.name).as_str(),
        HEADER_IMAGE_WIDTH,
        HEADER_IMAGE_HEIGHT,
    );

    let page = Page::new(
        Slug::new(&format!("/interests/games/{}/", game.id)),
        Some(title.as_str()),
        Some(description.as_str()),
    )
    .with_image(&image);

    let template = GameTemplate {
        page: &page,
        game,
        unlocked_achievements: unlocked_achievements.iter().collect(),
        locked_achievements: locked_achievements.iter().collect(),
        total_achievements,
    };

    render_page_with_template(state, &page, template).await
}
