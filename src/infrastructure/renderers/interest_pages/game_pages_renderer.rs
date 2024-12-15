use std::time::Duration;

use askama::Template;
use chrono::{DateTime, Utc};
use tokio::try_join;

use crate::domain::models::game::{Game, GameAchievmentLocked, GameAchievmentUnlocked};
use crate::domain::models::image::Image;
use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::site_config::PageImage;
use crate::domain::models::slug::Slug;
use crate::domain::models::steam::{SteamGameAchievementLocked, SteamGameAchievementUnlocked};
use crate::domain::models::{page::Page, steam::SteamGame};

use crate::domain::queries::games_queries::{
    find_all_games_by_achievment_unlocked_percentage, find_all_games_by_most_recently_played,
    GameWithAchievements,
};
use crate::domain::queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};
use crate::domain::repositories::{SteamAchievementsRepo, SteamGamesRepo};
use crate::domain::services::PageRenderingService;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::domain::state::State;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const RECENTLY_PLAYED_GAMES_COUNT: usize = 6;
const HEADER_IMAGE_WIDTH: u32 = 414;
const HEADER_IMAGE_HEIGHT: u32 = 193;

pub async fn render_games_pages(state: &impl State) -> Result<()> {
    let games = find_all_games_by_most_recently_played(state).await?;

    try_join!(
        render_games_list_page(state, &games),
        render_activity_page(state),
        render_games_list_most_achieved_page(state, &games),
    )?;

    for game in games {
        match game {
            Game::Steam(steam_game) => render_steam_game_page(state, &steam_game).await?,
            _ => {}
        }
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "interests/games/games_list.html")]
struct GamesListTemplate {
    page: Page,
    games_by_recently_played: Vec<Game>,
    games_by_most_played: Vec<Game>,
    total_games: usize,
    total_playtime: f32,
}

async fn render_games_list_page(state: &impl State, games: &[Game]) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games"),
        Some("Games"),
        Some("My Games"),
    );

    let mut games_by_recently_played = games.to_vec();
    let mut games_by_most_played = games.to_vec();

    games_by_most_played.sort_by(|a, b| b.playtime().partial_cmp(&a.playtime()).unwrap());

    let games_by_recently_played = games_by_recently_played
        .iter()
        .take(RECENTLY_PLAYED_GAMES_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    let total_games = games.len();
    let total_playtime = games.iter().map(|g| g.playtime_hours()).sum::<f32>();

    let template = GamesListTemplate {
        page,
        games_by_recently_played,
        games_by_most_played,
        total_games,
        total_playtime,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None)
        .await
}

#[derive(Template)]
#[template(path = "interests/games/games_list_most_achievements.html")]
struct GamesListMostAchievmentsTemplate {
    page: Page,
    games: Vec<GameWithAchievements>,
}

async fn render_games_list_most_achieved_page(state: &impl State, games: &[Game]) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games/achievements"),
        Some("Game Achievments"),
        Some("My Game Achievments"),
    );

    let games = find_all_games_by_achievment_unlocked_percentage(state)
        .await?
        .into_iter()
        .filter(|game| game.unlocked().len() > 0)
        .collect();

    let template = GamesListMostAchievmentsTemplate { page, games };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None)
        .await
}

#[derive(Template)]
#[template(path = "interests/games/steam/steam_game.html")]
struct SteamGameTemplate {
    page: Page,
    game: SteamGame,
    unlocked_achievements: Vec<SteamGameAchievementUnlocked>,
    locked_achievements: Vec<SteamGameAchievementLocked>,
    total_achievements: usize,
}

async fn render_steam_game_page(state: &impl State, game: &SteamGame) -> Result<()> {
    let unlocked_achievements = state
        .steam_achievements_repo()
        .find_all_unlocked_by_unlocked_date(game.id)
        .await?;

    let locked_achievements = state
        .steam_achievements_repo()
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
    .with_image(image);

    let template = SteamGameTemplate {
        page,
        game: game.clone(),
        unlocked_achievements,
        locked_achievements,
        total_achievements,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None)
        .await
}

#[derive(Template)]
#[template(path = "partials/omni_post_list_page.html")]
pub struct ActivityPageTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}

async fn render_activity_page(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_game_activity()).await?;

    let paginated = paginate(&omni_posts, 25);

    let page = Page::new(
        Slug::new("/interests/games/activity/"),
        Some("Game Activity"),
        Some("My Game Activity"),
    );

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = ActivityPageTemplate {
            page,
            posts: paginator_page.data.to_vec(),
        };

        state
            .page_rendering_service()
            .add_page(
                state,
                template.page.slug.clone(),
                template,
                paginator_page
                    .data
                    .first()
                    .map(|p| p.last_updated_at())
                    .flatten(),
            )
            .await?;
    }

    Ok(())
}
