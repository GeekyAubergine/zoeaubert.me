use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    application::events::Event,
    domain::games::games_models::{
        Game, GameAchievement, GameAchievementLocked, GameAchievementUnlocked,
    },
    get_json,
    infrastructure::{app_state::AppState, bus::job_runner::Job, config::Config},
    load_archive_file,
    prelude::Result,
    save_archive_file, GAMES_ARCHIVE_FILENAME, ONE_DAY_CACHE_PERIOD,
};

use super::fetch_games_data_from_steam_job::SteamOwnedGame;

const NO_REFETCH_DURATION: Duration = ONE_DAY_CACHE_PERIOD;

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
    description: String,
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

fn steam_last_played_to_datetime(last_played: u32) -> DateTime<Utc> {
    match DateTime::from_timestamp(last_played as i64 * 1000, 0) {
        Some(date) => date,
        None => Utc::now(),
    }
}

async fn load_data_for_steam_game(steam_game: &SteamOwnedGame, config: &Config) -> Result<Game> {
    let game_link = format!(
        "https://store.steampowered.com/app/{}/{}",
        steam_game.appid(),
        steam_game.name().replace(' ', "_")
    );

    let game_header_image = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        steam_game.appid()
    );

    let mut game = Game::new(
        steam_game.appid(),
        steam_game.name().to_string(),
        game_header_image,
        steam_game.playtime_forever(),
        steam_last_played_to_datetime(steam_game.rtime_last_played()),
        game_link,
        HashMap::new(),
    );

    match get_steam_game_data(steam_game.appid(), config).await {
        Ok(game_data) => {
            let player_achievements =
                get_steam_player_achievements(steam_game.appid(), config).await?;

            let global_achievement_percentage =
                get_steam_global_achievement_percentage(steam_game.appid(), config).await?;

            let mut achievements = HashMap::new();

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

                let mapped_achievement = match unlocked_date {
                    Some(unlocked_date) => GameAchievement::Unlocked(GameAchievementUnlocked::new(
                        achievement.name.clone(),
                        achievement.display_name,
                        achievement.description,
                        achievement.icon,
                        unlocked_date,
                        global_percentage,
                    )),
                    None => GameAchievement::Locked(GameAchievementLocked::new(
                        achievement.name.clone(),
                        achievement.display_name,
                        achievement.description,
                        achievement.icon_gray,
                        global_percentage,
                    )),
                };

                achievements.insert(achievement.name.clone(), mapped_achievement.clone());
            }

            game.set_achievements(achievements);

            Ok(game)
        }
        // So this seems weird but sometimes the API just returns `Game: {}` for some games that aren't really games :shrug: I don't care that much as it wont have the achievement data anyway
        Err(err) => Ok(game),
    }
}

#[derive(Debug)]
pub struct FetchGameDataFromSteamJob {
    game: SteamOwnedGame,
}

impl FetchGameDataFromSteamJob {
    pub fn new(game: SteamOwnedGame) -> Self {
        Self { game }
    }
}

#[async_trait]
impl Job for FetchGameDataFromSteamJob {
    fn name(&self) -> &str {
        "FetchGameDataFromSteamJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        if let Some(stored_game) = app_state.games_repo().get_game(self.game.appid()).await {
            if &steam_last_played_to_datetime(self.game.rtime_last_played())
                <= stored_game.last_played()
            {
                return Ok(());
            }
        }

        info!("Updating game: {}", self.game.name());

        let game_with_achievements = load_data_for_steam_game(&self.game, app_state.config()).await;

        // TODO Better error and use ?
        match game_with_achievements {
            Ok(game) => {
                app_state.games_repo().commit(game).await;
                app_state.dispatch_event(Event::GamesRepoUpdated).await?;
            }
            Err(err) => {
                println!("Error loading game data: {:?}", err);
                // TODO log
            }
        }

        Ok(())
    }
}
