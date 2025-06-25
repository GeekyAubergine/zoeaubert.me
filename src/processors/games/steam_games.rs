use std::{collections::HashMap, path::Path, time::Duration};

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::{
    domain::models::{
        games::steam::{
            SteamGame, SteamGameAchievement, SteamGameAchievementLocked,
            SteamGameAchievementUnlocked, SteamGameWithAchievements, SteamGames,
        },
        image::Image,
        slug::Slug,
    },
    prelude::*,
    services::{
        file_service::{FilePath, FileService},
        ServiceContext,
    },
};

const QUERY_KEY: &str = "steam_games";

const FILE_NAME: &str = "steam_games.json";

const GAMES_TO_IGNORE: &[u32] = &[
    219540, // Arma 2: Opertion Arrowhead - Beta (Obsolete)
];

const STEAM_OWNED_GAMES_URL: &str =
  "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?format=json&include_appinfo=true";

const STEAM_PLAYER_ACHEIVEMENTS_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?format=json";

const STEAM_GAME_DATA_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json";

const STEAM_GAME_GLOBAL_ACHIEMENT_PERCENTAGE_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?format=json";

// ---- Steam Game Data

fn make_steam_game_data_url(appid: u32) -> Url {
    format!(
        "{}&key={}&appid={}",
        STEAM_GAME_DATA_URL,
        dotenv!("STEAM_API_KEY"),
        appid
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameDataAcheivement {
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    description: Option<String>,
    icon: Url,
    #[serde(rename = "icongray")]
    icon_gray: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameDataAvaialableGameStats {
    achievements: Vec<SteamGameDataAcheivement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum SteamAvailableGameSchemaResponseWrapper {
    WithGame {
        #[serde(rename = "gameName")]
        game_name: String,
        #[serde(rename = "availableGameStats")]
        available_game_stats: SteamGameDataAvaialableGameStats,
    },
    WithoutGame {},
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamAvailableGameStatsResponse {
    game: SteamAvailableGameSchemaResponseWrapper,
}

async fn get_steam_game_data(
    ctx: &ServiceContext,
    appid: u32,
) -> Result<Vec<SteamGameDataAcheivement>> {
    let response = ctx
        .network
        .download_json::<SteamAvailableGameStatsResponse>(&make_steam_game_data_url(appid))
        .await?;

    match response.game {
        SteamAvailableGameSchemaResponseWrapper::WithGame {
            available_game_stats,
            ..
        } => Ok(available_game_stats.achievements),
        SteamAvailableGameSchemaResponseWrapper::WithoutGame {} => Ok(vec![]),
    }
}

// ---- Steam Game Player Achievements

fn make_get_player_achievements_url(appid: u32) -> Url {
    format!(
        "{}&key={}&appid={}&steamid={}",
        STEAM_PLAYER_ACHEIVEMENTS_URL,
        dotenv!("STEAM_API_KEY"),
        appid,
        dotenv!("STEAM_ID")
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGamePlayerAchievement {
    apiname: String,
    achieved: u8,
    unlocktime: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum SteamGetPlayerAchievementsResponseInner {
    Achievements {
        achievements: Vec<SteamGamePlayerAchievement>,
    },
    NoStats {
        error: Option<String>,
        success: bool,
        #[serde(rename = "gameName")]
        game_name: Option<String>,
        #[serde(rename = "steamID")]
        steam_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SteamGetPlayerStatsResponse {
    playerstats: SteamGetPlayerAchievementsResponseInner,
}

async fn get_steam_player_achievements(
    ctx: &ServiceContext,
    appid: u32,
) -> Result<Vec<SteamGamePlayerAchievement>> {
    let response = ctx
        .network
        .download_json::<SteamGetPlayerStatsResponse>(&make_get_player_achievements_url(appid))
        .await?;

    match response.playerstats {
        SteamGetPlayerAchievementsResponseInner::Achievements { achievements } => Ok(achievements),
        SteamGetPlayerAchievementsResponseInner::NoStats { .. } => Ok(vec![]),
    }
}

// ---- Steam Game Global Achievement Percentage

fn make_get_global_achievement_percentage_url(appid: u32) -> Url {
    format!(
        "{}&key={}&gameid={}",
        STEAM_GAME_GLOBAL_ACHIEMENT_PERCENTAGE_URL,
        dotenv!("STEAM_API_KEY"),
        appid
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameGlobalAchievement {
    name: String,
    percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SteamGetGlobalAchievementPercentagesResponseInner {
    achievements: Vec<SteamGameGlobalAchievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum SteamGetGlobalAchievementPercentagesResponse {
    WithAchievements {
        achievementpercentages: SteamGetGlobalAchievementPercentagesResponseInner,
    },
    Empty {},
}

async fn get_steam_global_achievement_percentage(
    ctx: &ServiceContext,
    appid: u32,
) -> Result<Vec<SteamGameGlobalAchievement>> {
    let response = ctx
        .network
        .download_json::<SteamGetGlobalAchievementPercentagesResponse>(
            &make_get_global_achievement_percentage_url(appid),
        )
        .await?;

    match response {
        SteamGetGlobalAchievementPercentagesResponse::Empty {} => Ok(vec![]),
        SteamGetGlobalAchievementPercentagesResponse::WithAchievements {
            achievementpercentages,
        } => Ok(achievementpercentages.achievements),
    }
}

pub async fn process_steam_game_achievements(
    ctx: &ServiceContext,
    game: SteamGame,
) -> Result<SteamGameWithAchievements> {
    info!(
        "Updating game achievements for game: {} [{}]",
        game.id, game.name
    );

    let player_achievements = get_steam_player_achievements(ctx, game.id).await?;

    let game_data = get_steam_game_data(ctx, game.id).await?;

    let global_achievement_percentage =
        get_steam_global_achievement_percentage(ctx, game.id).await?;

    let mut game = SteamGameWithAchievements::from_game(game);

    for achievement in game_data {
        // Sometimes the icon urls are just a directory
        if achievement.icon.as_str().ends_with("/") || achievement.icon_gray.as_str().ends_with("/")
        {
            continue;
        }

        let player_achievement = player_achievements
            .iter()
            .find(|player_achievement| player_achievement.apiname == achievement.name);

        let unlocked_date = match player_achievement {
            Some(player_achievement) => {
                if player_achievement.achieved == 1 {
                    DateTime::from_timestamp(player_achievement.unlocktime as i64, 0)
                } else {
                    None
                }
            }
            None => None,
        };

        let global_achievement = global_achievement_percentage
            .iter()
            .find(|global_achievement| global_achievement.name == achievement.name);

        let global_percentage = match global_achievement {
            Some(global_achievement) => global_achievement.percent,
            None => 0.0,
        };

        match unlocked_date {
            Some(unlocked_date) => {
                let path = &format!(
                    "/games/{}-{}-unlocked.jpg",
                    game.game.id,
                    achievement.name.replace(' ', "").replace('%', "")
                );

                let path = FilePath::cache(path);

                let image = ctx
                    .image
                    .copy_image_from_url(
                        ctx,
                        &achievement.icon,
                        &path,
                        &format!("{} achievement icon", achievement.display_name),
                    )
                    .await?;

                let achievement = SteamGameAchievementUnlocked::new(
                    achievement.name,
                    game.game.id,
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    image,
                    unlocked_date,
                    global_percentage,
                );

                game.add_unlocked_achievement(achievement);
            }
            None => {
                let path = &format!(
                    "/games/{}-{}-locked.jpg",
                    game.game.id,
                    achievement.name.replace(' ', "").replace('%', "")
                );

                let path = FilePath::cache(path);

                let icon = match &achievement.icon_gray.as_str().ends_with("/") {
                    true => achievement.icon,
                    false => achievement.icon_gray,
                };

                let image = ctx
                    .image
                    .copy_image_from_url(
                        ctx,
                        &icon,
                        &path,
                        &format!("{} achievement icon", achievement.display_name),
                    )
                    .await?;

                let achievement = SteamGameAchievementLocked::new(
                    achievement.name,
                    game.game.id,
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    image,
                    global_percentage,
                );

                game.add_locked_achievement(achievement);
            }
        };
    }

    Ok(game)
}

// ---- Games

fn make_get_games_url() -> Url {
    format!(
        "{}&key={}&steamid={}",
        STEAM_OWNED_GAMES_URL,
        dotenv!("STEAM_API_KEY"),
        dotenv!("STEAM_ID")
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamOwnedGame {
    appid: u32,
    name: String,
    playtime_forever: u32,
    img_icon_url: String,
    rtime_last_played: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct SteamGetOwnedGamesResponseInner {
    game_count: u32,
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Clone, Deserialize)]
struct SteamGetOwnedGamesResponse {
    response: SteamGetOwnedGamesResponseInner,
}

pub fn steam_last_played_to_datetime(last_played: u32) -> DateTime<Utc> {
    match DateTime::from_timestamp(last_played as i64, 0) {
        Some(date) => date,
        None => Utc::now(),
    }
}

async fn process_game(
    ctx: &ServiceContext,
    game: SteamOwnedGame,
    stored_game: Option<&SteamGameWithAchievements>,
) -> Result<SteamGameWithAchievements> {
    if let Some(stored_game) = stored_game {
        if steam_last_played_to_datetime(game.rtime_last_played) <= stored_game.game.last_played {
            return Ok(stored_game.clone());
        }
    }

    let game_header_image_cdn_path = &format!("games/{}-header.jpg", game.appid);

    let game_header_image_cdn_path = FilePath::cache(&game_header_image_cdn_path);

    let image_src_url: Url = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        game.appid
    )
    .parse()
    .unwrap();

    let image = ctx
        .image
        .copy_image_from_url(
            ctx,
            &image_src_url,
            &game_header_image_cdn_path,
            &format!("{} steam header image", &game.name),
        )
        .await?;

    let game = SteamGame::new(
        game.appid,
        game.name,
        image,
        Duration::from_secs(game.playtime_forever as u64),
        steam_last_played_to_datetime(game.rtime_last_played),
        format!("https://store.steampowered.com/app/{}", game.appid),
    );

    let game = process_steam_game_achievements(ctx, game).await?;

    Ok(game)
}

pub async fn process_steam_games(ctx: &ServiceContext) -> Result<SteamGames> {
    let mut data: SteamGames = FilePath::archive(FILE_NAME)
        .read_as_json_or_default()
        .await?;

    if !ctx.query_limiter.can_query_within_hour(QUERY_KEY).await? {
        return Ok(data);
    }

    info!("Processing steam games");

    let games = ctx
        .network
        .download_json::<SteamGetOwnedGamesResponse>(&make_get_games_url())
        .await?;

    for game in games.response.games {
        if GAMES_TO_IGNORE.contains(&game.appid) {
            continue;
        }

        let id = game.appid;

        data.add_game(process_game(ctx, game, data.find_game_by_id(id)).await?);

        break;
    }

    Ok(data)
}
