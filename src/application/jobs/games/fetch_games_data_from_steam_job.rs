use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::{
    application::events::Event, domain::models::game::Game, get_json, infrastructure::{
        app_state::AppState,
        bus::job_runner::{Job, JobPriority},
        config::Config,
    }, prelude::Result, GAMES_ARCHIVE_FILENAME, ONE_HOUR_CACHE_PERIOD
};

use super::fetch_game_data_from_steam_job::{
    steam_last_played_to_datetime, FetchGameAchievementsFromSteamJob,
};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

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

async fn process_game(state: &AppState, game: SteamOwnedGame) -> Result<()> {
    if let Some(stored_game) = state.games_repo().find_by_id(game.appid()).await? {
        if &steam_last_played_to_datetime(game.rtime_last_played())
            <= stored_game.last_played()
        {
            return Ok(());
        }
    }

    info!("Fething game: {}", game.name());

    let game_link = format!(
        "https://store.steampowered.com/app/{}/{}",
        game.appid(),
        game.name().replace(' ', "_")
    );

    let game_header_image = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        game.appid()
    );

    let game = Game::new(
        game.appid(),
        game.name().to_string(),
        game_header_image,
        game.playtime_forever(),
        steam_last_played_to_datetime(game.rtime_last_played()),
        game_link,
        Utc::now(),
    );

    state.games_repo().commit(&game).await?;

    state.dispatch_event(Event::GameUpdated { game_id: game.id() }).await?;

    Ok(())
}

#[async_trait]
impl Job for GamesDownloadDataJob {
    fn name(&self) -> &str {
        "GamesDownloadDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let last_fetch = app_state
            .games_repo()
            .find_most_recently_updated_date()
            .await?;

        if let Some(last_fetch) = last_fetch {
            if last_fetch + NO_REFETCH_DURATION > Utc::now() {
                debug!("Skipping fetching games data from steam");
                return Ok(());
            }
        }

        let games = app_state.games_repo().find_all_games().await;

        info!("Fetching steam games data");

        let steam_owned_games_response =
            get_json::<SteamGetOwnedGamesResponse>(&make_get_games_url(app_state.config())).await?;

        for steam_game in steam_owned_games_response.response.games {
            process_game(app_state, steam_game).await?;
        }

        Ok(())
    }
}
