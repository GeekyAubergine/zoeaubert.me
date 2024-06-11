use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::warn;

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

use super::fetch_game_data_from_steam_job::{steam_last_played_to_datetime, FetchGameDataFromSteamJob};

const NO_REFETCH_DURATION: Duration = ONE_DAY_CACHE_PERIOD;

const STEAM_OWNED_GAMES_URL: &str =
  "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?format=json&include_appinfo=true";

const STEAM_PLAYER_ACHEIVEMENTS_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?format=json";

const STEAM_GAME_DATA_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json";

const STEAM_GAME_GLOBAL_ACHIEMENT_PERCENTAGE_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/?format=json";

// ---- Steam Game

fn make_get_games_url(config: &Config) -> String {
    format!(
        "{}&key={}&steamid={}",
        STEAM_OWNED_GAMES_URL,
        config.steam().api_key(),
        config.steam().steam_id()
    )
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SteamOwnedGame {
    appid: u32,
    name: String,
    playtime_forever: u32,
    img_icon_url: String,
    rtime_last_played: u32,
}

impl SteamOwnedGame {
    pub fn appid(&self) -> u32 {
        self.appid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn playtime_forever(&self) -> u32 {
        self.playtime_forever
    }

    pub fn img_icon_url(&self) -> &str {
        &self.img_icon_url
    }

    pub fn rtime_last_played(&self) -> u32 {
        self.rtime_last_played
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetOwnedGamesResponseInner {
    game_count: u32,
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SteamGetOwnedGamesResponse {
    response: SteamGetOwnedGamesResponseInner,
}

async fn get_steam_owned_games(config: &Config) -> Result<HashMap<u32, SteamOwnedGame>> {
    let response = get_json::<SteamGetOwnedGamesResponse>(&make_get_games_url(config)).await?;

    Ok(response
        .response
        .games
        .into_iter()
        .map(|game| (game.appid, game))
        .collect())
}

#[derive(Debug)]
pub struct GamesDownloadDataJob;
impl GamesDownloadDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for GamesDownloadDataJob {
    fn name(&self) -> &str {
        "GamesDownloadDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let last_updated = app_state.games_repo().get_last_updated().await;

        if last_updated + NO_REFETCH_DURATION > Utc::now() {
            return Ok(());
        }

        let last_updated_as_rtime = last_updated.timestamp() as u32;

        let steam_owned_games_response =
            get_json::<SteamGetOwnedGamesResponse>(&make_get_games_url(app_state.config())).await?;

        for steam_game in steam_owned_games_response.response.games {
            // Skip games that have not been played since the last update
            if let Some(stored_game) = app_state.games_repo().get_game(steam_game.appid()).await {
                if &steam_last_played_to_datetime(steam_game.rtime_last_played())
                    <= stored_game.last_played()
                {
                    continue;
                }
            }

            app_state
                .dispatch_job(FetchGameDataFromSteamJob::new(steam_game))
                .await?;
        }

        Ok(())
    }
}
