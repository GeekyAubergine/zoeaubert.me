use std::time::Duration;

use askama::Template;
use chrono::{DateTime, Utc};
use tokio::try_join;

use crate::domain::models::games::steam::{
    SteamGame, SteamGameAchievementLocked, SteamGameAchievementUnlocked, SteamGameWithAchievements,
};
use crate::domain::models::games::Game;
use crate::domain::models::image::Image;
use crate::domain::models::page::Page;
use crate::domain::models::post::Post;
use crate::domain::models::post::PostFilter;
use crate::domain::models::site_config::PageImage;
use crate::domain::models::slug::Slug;

use crate::prelude::*;
use crate::renderers::RendererContext;
use crate::utils::paginator::paginate;

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

const RECENTLY_PLAYED_GAMES_COUNT: usize = 6;
const HEADER_IMAGE_WIDTH: u32 = 414;
const HEADER_IMAGE_HEIGHT: u32 = 193;

pub async fn render_games_pages(context: &RendererContext) -> Result<()> {
    let games = context.data.games.find_by_most_recently_played();

    try_join!(
        render_games_list_page(context),
        render_activity_page(context),
        render_games_list_most_achieved_page(context),
    )?;

    for game in games {
        match game {
            Game::Steam(steam_game) => render_steam_game_page(context, &steam_game).await?,
            _ => {}
        }
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "interests/games/games_list.html")]
struct GamesListTemplate<'t> {
    page: Page,
    games_by_recently_played: Vec<&'t Game>,
    games_by_most_played: Vec<&'t Game>,
    total_games: usize,
    total_playtime: f32,
}

async fn render_games_list_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games"),
        Some("Games"),
        Some("My Games".to_string()),
    );

    let mut games_by_recently_played = context.data.games.find_by_most_recently_played().to_vec();
    let mut games_by_most_played = context.data.games.find_by_most_most_played().to_vec();

    let games_by_recently_played = games_by_recently_played
        .iter()
        .take(RECENTLY_PLAYED_GAMES_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    let total_games = games_by_recently_played.len();
    let total_playtime = games_by_recently_played
        .iter()
        .map(|g| g.playtime_hours())
        .sum::<f32>();

    let template = GamesListTemplate {
        page,
        games_by_recently_played,
        games_by_most_played,
        total_games,
        total_playtime,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}

#[derive(Template)]
#[template(path = "interests/games/games_list_most_achievements.html")]
struct GamesListMostAchievmentsTemplate<'t> {
    page: Page,
    games: Vec<&'t Game>,
}

async fn render_games_list_most_achieved_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games/achievements"),
        Some("Game Achievements"),
        Some("My Game Achievements".to_string()),
    );

    let games = context
        .data
        .games
        .find_by_most_highest_achievement_unlocked_percentage();

    let template = GamesListMostAchievmentsTemplate { page, games };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}

#[derive(Template)]
#[template(path = "interests/games/steam/steam_game.html")]
struct SteamGameTemplate<'t> {
    page: Page,
    game: &'t SteamGame,
    unlocked_achievements: Vec<&'t SteamGameAchievementUnlocked>,
    locked_achievements: Vec<&'t SteamGameAchievementLocked>,
    total_achievements: usize,
}

async fn render_steam_game_page(
    context: &RendererContext,
    game: &SteamGameWithAchievements,
) -> Result<()> {
    let unlocked_achievements = game.find_all_unlocked_by_unlocked_date();

    let locked_achievements = game.find_all_locked_by_name();

    let total_achievements = unlocked_achievements.len() + locked_achievements.len();

    let title = format!("{} Game Stats", game.game.name);

    let description = match total_achievements {
        0 => format!("{}h playtime", game.game.playtime_hours().format(1, true),),
        _ => format!(
            "{}h playtime, {}/{} achievements",
            game.game.playtime_hours().format(1, true),
            game.unlocked_achievement_count(),
            total_achievements,
        ),
    };

    let image = PageImage::new(
        game.game.header_image.cdn_url().as_str(),
        format!("{} Steam header image", game.game.name).as_str(),
        HEADER_IMAGE_WIDTH,
        HEADER_IMAGE_HEIGHT,
    );

    let page = Page::new(
        Slug::new(&format!("/interests/games/{}/", game.game.id)),
        Some(title.as_str()),
        Some(description),
    )
    .with_image(image);

    let template = SteamGameTemplate {
        page,
        game: &game.game,
        unlocked_achievements,
        locked_achievements,
        total_achievements: total_achievements as usize,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}

#[derive(Template)]
#[template(path = "posts/post_list/post_list_page.html")]
pub struct ActivityPageTemplate<'t> {
    page: Page,
    posts: Vec<&'t Post>,
}

async fn render_activity_page(context: &RendererContext) -> Result<()> {
    let omni_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_game_activity());

    let paginated = paginate(&omni_posts, 25);

    let page = Page::new(
        Slug::new("/interests/games/activity/"),
        Some("Game Activity"),
        Some("My Game Activity".to_string()),
    );

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = ActivityPageTemplate {
            page,
            posts: paginator_page.data.to_vec(),
        };

        context
            .renderer
            .render_page(&template.page.slug, &template, None)
            .await?;
    }

    Ok(())
}
