use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::application::commands::steam_commands::update_steam_game_achievements_command::update_steam_game_achievements_command;
use crate::domain::models::steam::SteamGame;
use crate::domain::repositories::SteamGamesRepo;
use crate::domain::services::{CdnService, ImageService, NetworkService, QueryLimitingService};
use crate::domain::state::State;

use crate::{prelude::*};

const QUERY: &str = "games";

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

// ---- Steam Game

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SteamGetOwnedGamesResponseInner {
    game_count: u32,
    games: Vec<SteamOwnedGame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SteamGetOwnedGamesResponse {
    response: SteamGetOwnedGamesResponseInner,
}

pub fn steam_last_played_to_datetime(last_played: u32) -> DateTime<Utc> {
    match DateTime::from_timestamp(last_played as i64, 0) {
        Some(date) => date,
        None => Utc::now(),
    }
}

async fn process_game(state: &impl State, game: SteamOwnedGame) -> Result<()> {
    if let Some(stored_game) = state.steam_games_repo().find_by_game_id(game.appid).await? {
        if steam_last_played_to_datetime(game.rtime_last_played) <= stored_game.last_played {
            return Ok(());
        }
    }

    let game_header_image_cdn_path = &format!("games/{}-header.jpg", game.appid);

    let game_header_image_cdn_path = Path::new(&game_header_image_cdn_path);

    let image_src_url: Url = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        game.appid
    )
    .parse()
    .unwrap();

    let image = state
        .image_service()
        .copy_image_from_url(
            state,
            &image_src_url,
            &game_header_image_cdn_path,
            &format!("{} steam header image", &game.name),
        )
        .await?;

    let game = SteamGame::new(
        game.appid,
        game.name,
        image,
        game.playtime_forever,
        steam_last_played_to_datetime(game.rtime_last_played),
        format!("https://store.steampowered.com/app/{}", game.appid),
    );

    update_steam_game_achievements_command(state, &game).await?;

    state.steam_games_repo().commit(&game).await?;

    Ok(())
}

pub async fn update_steam_games_command(state: &impl State) -> Result<()> {
    if !state.query_limiting_service().can_query_within_hour(QUERY).await? {
        return Ok(());
    }

    info!("Fetching steam games data");

    let steam_owned_games_response = state
        .network_service()
        .download_json::<SteamGetOwnedGamesResponse>(&make_get_games_url())
        .await?;

    for steam_game in steam_owned_games_response.response.games {
        if GAMES_TO_IGNORE.contains(&steam_game.appid) {
            continue;
        }

        process_game(state, steam_game).await?;
    }

    Ok(())
}
