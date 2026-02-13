use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use url::Url;

use crate::{
    config::CONFIG,
    domain::models::{
        games::steam::{SteamGame, SteamGameWithAchievements, SteamGames},
        image::Image,
    },
    prelude::*,
    processors::games::processor_steam_games::process_steam_game_achievement::process_steam_game_achievements,
    services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile, WritableFile},
        media_service::MediaService,
    },
};

pub mod process_steam_game_achievement;

const QUERY_KEY: &str = "steam_games";

const FILE_NAME: &str = "steam_games.json";

const GAMES_TO_IGNORE: &[u32] = &[
    219540, // Arma 2: Opertion Arrowhead - Beta (Obsolete)
];

const STEAM_OWNED_GAMES_URL: &str = "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?format=json&include_appinfo=true";
// ---- Games

fn make_get_games_url() -> Url {
    format!(
        "{}&key={}&steamid={}",
        STEAM_OWNED_GAMES_URL, CONFIG.steam.api_key, CONFIG.steam.user_id
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

fn get_game_header_image(
    ctx: &ServiceContext,
    game: &SteamOwnedGame,
    cdn_file: &CdnFile,
) -> Result<Image> {
    let image_src_url: Url = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        game.appid
    )
    .parse()
    .unwrap();

    if let Ok(image) = MediaService::image_from_url(
        ctx,
        &image_src_url,
        cdn_file,
        &format!("{} steam header image", &game.name),
        None,
        None,
    ) {
        return Ok(image);
    }

    let image_src_url: Url = format!(
        "https://media.steampowered.com/steamcommunity/public/images/apps/{}/{}.jpg",
        game.appid, game.img_icon_url,
    )
    .parse()
    .unwrap();

    MediaService::image_from_url(
        ctx,
        &image_src_url,
        cdn_file,
        &format!("{} steam header image", &game.name),
        None,
        None,
    )
}

#[instrument(err, skip_all, fields(game.id=game.appid,game.name=game.name))]
fn process_game(
    ctx: &ServiceContext,
    game: &SteamOwnedGame,
    stored_game: Option<&SteamGameWithAchievements>,
) -> Result<SteamGameWithAchievements> {
    if let Some(stored_game) = stored_game
        && steam_last_played_to_datetime(game.rtime_last_played) <= stored_game.game.last_played
    {
        return Ok(stored_game.clone());
    }

    info!("Processing game [{}]", game.name);

    let game_header_cdn_file = CdnFile::from_path(&format!("games/{}-header.jpg", game.appid));

    let image = get_game_header_image(ctx, game, &game_header_cdn_file)?;

    let game = SteamGame::new(
        game.appid,
        game.name.clone(),
        image,
        Duration::from_mins(game.playtime_forever as u64),
        steam_last_played_to_datetime(game.rtime_last_played),
        format!("https://store.steampowered.com/app/{}", game.appid),
    );

    let game = process_steam_game_achievements(ctx, game)?;

    Ok(game)
}

pub fn load_steam_games(ctx: &ServiceContext) -> Result<SteamGames> {
    let file = FileService::archive(FILE_NAME.into());

    let mut data: SteamGames = file.read_json_or_default()?;

    if !ctx.query_limiter.can_query_within_day(QUERY_KEY)? {
        return Ok(data);
    }

    info!("Processing steam games");

    let games = ctx
        .network
        .download_json::<SteamGetOwnedGamesResponse>(&make_get_games_url())?;

    for game in games.response.games {
        if GAMES_TO_IGNORE.contains(&game.appid) {
            continue;
        }

        let stored = data.find_game_by_id(game.appid);

        match process_game(ctx, &game, stored) {
            Ok(game) => {
                let should_save = match stored {
                    Some(stored) => !stored.eq(&game),
                    None => true,
                };

                data.add_game(game);

                if should_save {
                    file.write_json(&data)?;
                }
            }
            Err(_) => {
                let appid = &game.appid;
                let name = &game.name;
                warn!("Unable to process game [{appid}] [{name}]");
            }
        }
    }

    Ok(data)
}
