use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::{
    application::events::Event,
    domain::models::{
        game::Game,
        game_achievement::{GameAchievement, GameAchievementLocked, GameAchievementUnlocked},
    },
    error::GameError,
    get_json,
    infrastructure::{app_state::AppState, bus::job_runner::Job, config::Config},
    prelude::Result,
    GAMES_ARCHIVE_FILENAME, ONE_DAY_CACHE_PERIOD,
};

use super::fetch_games_data_from_steam_job::SteamOwnedGame;

const STEAM_OWNED_GAMES_URL: &str =
  "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?format=json&include_appinfo=true";

const STEAM_PLAYER_ACHEIVEMENTS_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?format=json";

const STEAM_GAME_DATA_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json";

const STEAM_GAME_GLOBAL_ACHIEMENT_PERCENTAGE_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?format=json";

// ---- Steam Game Data

fn make_steam_game_data_url(appid: u32, config: &Config) -> String {
    format!(
        "{}&key={}&appid={}",
        STEAM_GAME_DATA_URL,
        config.steam().api_key(),
        appid
    )
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamGameDataAcheivement {
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    description: Option<String>,
    icon: String,
    #[serde(rename = "icongray")]
    icon_gray: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamGameDataAvaialableGameStats {
    achievements: Vec<SteamGameDataAcheivement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamAvailableGameSchemaResponse {
    #[serde(rename = "gameName")]
    game_name: String,
    #[serde(rename = "availableGameStats")]
    available_game_stats: SteamGameDataAvaialableGameStats,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamAvailableGameStatsResponse {
    game: SteamAvailableGameSchemaResponse,
}

async fn get_steam_game_data(appid: u32, config: &Config) -> Result<Vec<SteamGameDataAcheivement>> {
    let response =
        get_json::<SteamAvailableGameStatsResponse>(&make_steam_game_data_url(appid, config))
            .await?;

    Ok(response.game.available_game_stats.achievements)
}

// ---- Steam Game Player Achievements

fn make_get_player_achievements_url(appid: u32, config: &Config) -> String {
    format!(
        "{}&key={}&appid={}&steamid={}",
        STEAM_PLAYER_ACHEIVEMENTS_URL,
        config.steam().api_key(),
        appid,
        config.steam().steam_id()
    )
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamGamePlayerAchievement {
    apiname: String,
    achieved: u8,
    unlocktime: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetPlayerAchievementsResponseInner {
    achievements: Vec<SteamGamePlayerAchievement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetPlayerStatsResponse {
    playerstats: SteamGetPlayerAchievementsResponseInner,
}

async fn get_steam_player_achievements(
    appid: u32,
    config: &Config,
) -> Result<Vec<SteamGamePlayerAchievement>> {
    let response =
        get_json::<SteamGetPlayerStatsResponse>(&make_get_player_achievements_url(appid, config))
            .await?;

    Ok(response.playerstats.achievements)
}

// ---- Steam Game Global Achievement Percentage

fn make_get_global_achievement_percentage_url(appid: u32, config: &Config) -> String {
    format!(
        "{}&key={}&gameid={}",
        STEAM_GAME_GLOBAL_ACHIEMENT_PERCENTAGE_URL,
        config.steam().api_key(),
        appid
    )
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamGameGlobalAchievement {
    name: String,
    percent: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetGlobalAchievementPercentagesResponseInner {
    achievements: Vec<SteamGameGlobalAchievement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetGlobalAchievementPercentagesResponse {
    achievementpercentages: SteamGetGlobalAchievementPercentagesResponseInner,
}

async fn get_steam_global_achievement_percentage(
    appid: u32,
    config: &Config,
) -> Result<Vec<SteamGameGlobalAchievement>> {
    let response = get_json::<SteamGetGlobalAchievementPercentagesResponse>(
        &make_get_global_achievement_percentage_url(appid, config),
    )
    .await?;

    Ok(response.achievementpercentages.achievements)
}

// ----

pub fn steam_last_played_to_datetime(last_played: u32) -> DateTime<Utc> {
    match DateTime::from_timestamp(last_played as i64, 0) {
        Some(date) => date,
        None => Utc::now(),
    }
}

#[derive(Debug)]
pub struct FetchGameAchievementsFromSteamJob {
    game_id: u32,
}

impl FetchGameAchievementsFromSteamJob {
    pub fn new(game_id: u32) -> Self {
        Self { game_id }
    }
}

#[async_trait]
impl Job for FetchGameAchievementsFromSteamJob {
    fn name(&self) -> &str {
        "FetchGameAchievementsFromSteamJob"
    }

    async fn run(&self, state: &AppState) -> Result<()> {
        let game = state
            .games_repo()
            .find_by_id(self.game_id)
            .await?
            .ok_or(GameError::game_not_found(self.game_id))?;

        info!("Fething game achievments: {} [{}]", game.name(), game.id());
        let player_achievements = get_steam_player_achievements(game.id(), state.config()).await?;

        let game_data = get_steam_game_data(game.id(), state.config()).await?;


        let global_achievement_percentage =
            get_steam_global_achievement_percentage(game.id(), state.config()).await?;

        for achievement in game_data {
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
                Some(unlocked_date) => GameAchievement::Unlocked(GameAchievementUnlocked::new(
                    achievement.name.clone(),
                    game.id(),
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    achievement.icon,
                    unlocked_date,
                    global_percentage,
                    Utc::now(),
                )),
                None => GameAchievement::Locked(GameAchievementLocked::new(
                    achievement.name.clone(),
                    game.id(),
                    achievement.display_name,
                    achievement.description.unwrap_or("".to_string()),
                    achievement.icon_gray,
                    global_percentage,
                    Utc::now(),
                )),
            };

            state.game_achievements_repo().commit(&achievment).await?;
        }

        state.dispatch_event(Event::GamesRepoUpdated).await?;

        Ok(())
    }
}
