use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::application::commands::games_commands::update_game_achievements_command::update_game_achievements_command;
use crate::domain::models::games::Game;
use crate::domain::queries::games_queries::{
    commit_game, find_game_by_id, find_games_last_updated_at,
};
use crate::domain::services::CdnService;
use crate::domain::state::State;

use crate::infrastructure::utils::image_utils::image_from_url;
use crate::infrastructure::utils::networking::download_json;
use crate::{prelude::*, ONE_HOUR_PERIOD};

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

async fn process_game(
    state: &impl State,
    client: &reqwest::Client,
    game: SteamOwnedGame,
) -> Result<()> {
    if let Some(stored_game) = find_game_by_id(state, game.appid).await? {
        if steam_last_played_to_datetime(game.rtime_last_played) <= stored_game.last_played
        {
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

    let image = image_from_url(
        state,
        &image_src_url,
        &game_header_image_cdn_path,
        &format!("{} steam header image", &game.name),
    )
    .await?;

    let game = Game::new(
        game.appid,
        game.name,
        image,
        game.playtime_forever,
        steam_last_played_to_datetime(game.rtime_last_played),
        format!("https://store.steampowered.com/app/{}", game.appid),
    );

    // Do achievments first as if they fail we don't want to commit the game
    update_game_achievements_command(state, &client, &game).await?;

    commit_game(state, &game).await?;

    Ok(())
}

pub async fn update_games_command(state: &impl State) -> Result<()> {
    // if let Some(last_updated) = find_games_last_updated_at(state).await? {
    //     if last_updated + ONE_HOUR_PERIOD > Utc::now() {
    //         return Ok(());
    //     }
    // }

    info!("Fetching steam games data");

    let client = reqwest::Client::new();

    let steam_owned_games_response =
        download_json::<SteamGetOwnedGamesResponse>(&client, &make_get_games_url()).await?;

    for steam_game in steam_owned_games_response.response.games {
        if GAMES_TO_IGNORE.contains(&steam_game.appid) {
            continue;
        }

        process_game(state, &client, steam_game).await?;
    }

    Ok(())
}
