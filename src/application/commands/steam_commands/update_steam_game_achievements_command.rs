use std::path::Path;

use chrono::DateTime;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::domain::models::steam::{
    SteamGame, SteamGameAchievement, SteamGameAchievementLocked, SteamGameAchievementUnlocked,
};
use crate::domain::repositories::{SteamAchievementsRepo, SteamGamesRepo, Profiler};
use crate::domain::services::{CdnService, ImageService, NetworkService};
use crate::domain::state::State;
use crate::error::GameError;

use crate::prelude::*;

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
    state: &impl State,
    appid: u32,
) -> Result<Vec<SteamGameDataAcheivement>> {
    let response = state
        .network_service()
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
    state: &impl State,
    appid: u32,
) -> Result<Vec<SteamGamePlayerAchievement>> {
    let response = state
        .network_service()
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
    state: &impl State,
    appid: u32,
) -> Result<Vec<SteamGameGlobalAchievement>> {
    let response = state
        .network_service()
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

pub async fn update_steam_game_achievements_command(state: &impl State, game: &SteamGame) -> Result<()> {
    info!(
        "Updating game achievements for game: {} [{}]",
        game.id, game.name
    );

    let player_achievements = get_steam_player_achievements(state, game.id).await?;

    let game_data = get_steam_game_data(state, game.id).await?;

    let global_achievement_percentage =
        get_steam_global_achievement_percentage(state, game.id).await?;

    for achievement in game_data {
        state.profiler().entity_processed().await?;

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

        let achievment = match unlocked_date {
            Some(unlocked_date) => {
                let path = &format!(
                    "/games/{}-{}-unlocked.jpg",
                    game.id,
                    achievement.name.replace(' ', "").replace('%', "")
                );

                let path = Path::new(&path);

                let image = state
                    .image_service()
                    .copy_image_from_url(
                        state,
                        &achievement.icon,
                        &path,
                        &format!("{} achievement icon", achievement.display_name),
                    )
                    .await?;

                SteamGameAchievement::Unlocked(SteamGameAchievementUnlocked::new(
                    achievement.name,
                    game.id,
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    image,
                    unlocked_date,
                    global_percentage,
                ))
            }
            None => {
                let path = &format!(
                    "/games/{}-{}-locked.jpg",
                    game.id,
                    achievement.name.replace(' ', "").replace('%', "")
                );

                let path = Path::new(&path);

                let icon = match &achievement.icon_gray.as_str().ends_with("/") {
                    true => achievement.icon,
                    false => achievement.icon_gray,
                };

                let image = state
                    .image_service()
                    .copy_image_from_url(
                        state,
                        &icon,
                        &path,
                        &format!("{} achievement icon", achievement.display_name),
                    )
                    .await?;

                SteamGameAchievement::Locked(SteamGameAchievementLocked::new(
                    achievement.name,
                    game.id,
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    image,
                    global_percentage,
                ))
            }
        };

        state
            .steam_achievements_repo()
            .commit(&game, &achievment)
            .await?;
    }

    Ok(())
}
