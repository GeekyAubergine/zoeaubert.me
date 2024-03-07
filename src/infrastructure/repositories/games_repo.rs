use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
    vec,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinSet};

use crate::{
    domain::models::game::{Game, GameAchievement},
    get_json,
    infrastructure::config::Config,
    prelude::*,
    ONE_DAY_CACHE_PERIOD, ONE_HOUR_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_DAY_CACHE_PERIOD;

const STEAM_OWNED_GAMES_URL: &str =
  "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?format=json&include_appinfo=true";

const STEAM_PLAYER_ACHEIVEMENTS_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?format=json";

const STEAM_GAME_DATA_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json";

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

async fn load_data_for_steam_game(steam_game: SteamOwnedGame, config: Config) -> Result<Game> {
    let game_link = format!(
        "https://store.steampowered.com/app/{}/{}",
        steam_game.appid,
        steam_game.name.replace(' ', "_")
    );

    let game_header_image = format!(
        "https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg",
        steam_game.appid
    );

    let mut game = Game::new(
        steam_game.appid,
        steam_game.name,
        game_header_image,
        steam_game.playtime_forever,
        steam_game.rtime_last_played,
        game_link,
        HashMap::new(),
    );

    match get_steam_game_data(steam_game.appid, &config).await {
        Ok(game_data) => {
            let player_achievements =
                get_steam_player_achievements(steam_game.appid, &config).await?;

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

                achievements.insert(
                    achievement.name.clone(),
                    GameAchievement::new(
                        achievement.name.clone(),
                        achievement.display_name,
                        achievement.description,
                        achievement.icon,
                        achievement.icon_gray,
                        unlocked_date,
                    ),
                );
            }

            game.set_achievements(achievements);

            Ok(game)
        }
        // So this seems weird but sometimes the API just returns `Game: {}` for some games that aren't really games :shrug: I don't care that much as it wont have the achievement data anyway
        Err(err) => Ok(game),
    }
}

#[derive(Debug, Clone, Default)]
pub struct GamesRepo {
    games: Arc<RwLock<HashMap<u32, Game>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl GamesRepo {
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
            last_updated: Arc::new(RwLock::new(UNIX_EPOCH.into())),
        }
    }

    pub fn from_archive(archive: GameRepoArchive) -> Self {
        Self {
            games: Arc::new(RwLock::new(archive.games)),
            last_updated: Arc::new(RwLock::new(archive.last_updated)),
        }
    }

    pub async fn reload(&self, config: &Config) -> Result<()> {
        let last_updated = *self.last_updated.read().await;

        if last_updated + NO_REFETCH_DURATION > Utc::now() {
            return Ok(());
        }

        let steam_owned_games_response =
            get_json::<SteamGetOwnedGamesResponse>(&make_get_games_url(config)).await?;

        // let mut steam_game_data_tasks = JoinSet::new();

        // for game in steam_owned_games_response.response.games {
        //     let game_with_achievements = load_data_for_steam_game(game, config.clone());

        //     steam_game_data_tasks.spawn(game_with_achievements);
        // }

        let mut steam_games = HashMap::new();

        // while let Some(game) = steam_game_data_tasks.join_next().await {
        //     if let Ok(Ok(game)) = game {
        //         steam_games.insert(game.id(), game);
        //     } else {
        //         println!("Error loading game data");
        //         // TODO log
        //     }
        // }

        for game in steam_owned_games_response.response.games {
            let game_with_achievements = load_data_for_steam_game(game, config.clone()).await;

            match game_with_achievements {
                Ok(game) => {
                    steam_games.insert(game.id(), game);
                }
                Err(err) => {
                    println!("Error loading game data: {:?}", err);
                    // TODO log
                    break;
                }
            }
        }

        let mut steam_games_ref = self.games.write().await;

        *steam_games_ref = steam_games;

        let mut last_updated = self.last_updated.write().await;

        *last_updated = Utc::now();

        Ok(())
    }

    pub async fn get_archived(&self) -> GameRepoArchive {
        let games = self.games.read().await;

        GameRepoArchive {
            games: games.clone(),
            last_updated: *self.last_updated.read().await,
        }
    }

    pub async fn get_all_games(&self) -> HashMap<u32, Game> {
        let games = self.games.read().await;

        games
            .iter()
            .map(|(key, game)| (*key, game.clone().into()))
            .collect()
    }

    pub async fn get_games_by_most_recently_played(&self) -> Vec<u32> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by(|a, b| b.last_played().cmp(&a.last_played()));

        games_array
            .iter()
            .map(|game| game.id())
            .collect::<Vec<u32>>()
    }

    pub async fn get_games_by_most_played(&self) -> Vec<u32> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by(|a, b| b.playtime().cmp(&a.playtime()));

        games_array
            .iter()
            .map(|game| game.id())
            .collect::<Vec<u32>>()
    }

    pub async fn get_games_by_most_completed_achievements(&self) -> Vec<u32> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by(|a, b| {
            b.achievements_unlocked_count()
                .cmp(&a.achievements_unlocked_count())
        });

        games_array
            .iter()
            .map(|game| game.id())
            .collect::<Vec<u32>>()
    }

    pub async fn get_total_play_time(&self) -> u32 {
        let games = self.games.read().await;

        games.values().map(|game| game.playtime()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRepoArchive {
    games: HashMap<u32, Game>,
    last_updated: DateTime<Utc>,
}
