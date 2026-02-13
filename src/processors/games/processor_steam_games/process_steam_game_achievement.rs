use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use crate::{
    config::CONFIG,
    domain::models::games::steam::{
        SteamGame, SteamGameAchievement, SteamGameAchievementLocked, SteamGameAchievementUnlocked,
        SteamGameWithAchievements,
    },
    prelude::*,
    processors::tasks::{Task, run_tasks},
    services::{ServiceContext, cdn_service::CdnFile, media_service::MediaService},
};

const STEAM_PLAYER_ACHEIVEMENTS_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?format=json";

const STEAM_GAME_DATA_URL: &str =
    "http://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?format=json";

fn make_steam_game_data_url(appid: u32) -> Url {
    format!(
        "{}&key={}&appid={}",
        STEAM_GAME_DATA_URL, CONFIG.steam.api_key, appid
    )
    .parse()
    .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGameDataAchievement {
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
    achievements: Vec<SteamGameDataAchievement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum SteamAvailableGameSchemaResponseWrapper {
    WithGame {
        #[serde(rename = "availableGameStats")]
        available_game_stats: SteamGameDataAvaialableGameStats,
    },
    WithoutGame {},
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamAvailableGameStatsResponse {
    game: SteamAvailableGameSchemaResponseWrapper,
}

fn get_steam_game_data(ctx: &ServiceContext, appid: u32) -> Result<Vec<SteamGameDataAchievement>> {
    let response = ctx
        .network
        .download_json::<SteamAvailableGameStatsResponse>(&make_steam_game_data_url(appid))?;

    match response.game {
        SteamAvailableGameSchemaResponseWrapper::WithGame {
            available_game_stats,
            ..
        } => Ok(available_game_stats.achievements),
        SteamAvailableGameSchemaResponseWrapper::WithoutGame {} => Ok(vec![]),
    }
}

fn make_get_player_achievements_url(appid: u32) -> Url {
    format!(
        "{}&key={}&appid={}&steamid={}",
        STEAM_PLAYER_ACHEIVEMENTS_URL, CONFIG.steam.api_key, appid, CONFIG.steam.user_id,
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

fn get_steam_player_achievements(
    ctx: &ServiceContext,
    appid: u32,
) -> Result<Vec<SteamGamePlayerAchievement>> {
    let response = ctx
        .network
        .download_json::<SteamGetPlayerStatsResponse>(&make_get_player_achievements_url(appid))?;

    match response.playerstats {
        SteamGetPlayerAchievementsResponseInner::Achievements { achievements } => Ok(achievements),
        SteamGetPlayerAchievementsResponseInner::NoStats { .. } => Ok(vec![]),
    }
}

pub fn process_steam_game_achievements(
    ctx: &ServiceContext,
    game: SteamGame,
) -> Result<SteamGameWithAchievements> {
    info!(
        "Updating game achievements for game: {} [{}]",
        game.id, game.name
    );

    let player_achievements = get_steam_player_achievements(ctx, game.id)?;

    let game_data = get_steam_game_data(ctx, game.id)?;

    let mut game = SteamGameWithAchievements::from_game(game);

    let achievement_jobs = game_data
        .into_iter()
        .filter_map(|achievement| {
            if achievement.icon.as_str().ends_with("/") {
                return None;
            }

            Some(TaskProcessSteamGameAchievement {
                game: &game.game,
                player_achievements: &player_achievements,
                achievement,
            })
        })
        .collect();

    let achievments = run_tasks(achievement_jobs, ctx)?;

    for achievement in achievments {
        game.add_achievment(achievement);
    }

    Ok(game)
}

struct TaskProcessSteamGameAchievement<'l> {
    game: &'l SteamGame,
    achievement: SteamGameDataAchievement,
    player_achievements: &'l Vec<SteamGamePlayerAchievement>,
}

impl<'l> Task for TaskProcessSteamGameAchievement<'l> {
    type Output = SteamGameAchievement;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        let player_achievement = self
            .player_achievements
            .iter()
            .find(|player_achievement| player_achievement.apiname == self.achievement.name);

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

        match unlocked_date {
            Some(unlocked_date) => {
                let cdn_file = CdnFile::from_str(&format!(
                    "/games/{}-{}-unlocked.jpg",
                    self.game.id,
                    self.achievement.name.replace(' ', "").replace('%', "")
                ));

                let image = MediaService::image_from_url(
                    ctx,
                    &self.achievement.icon,
                    &cdn_file,
                    &format!("{} achievement icon", self.achievement.display_name),
                    None,
                    None,
                )?;

                Ok(SteamGameAchievement::Unlocked(
                    SteamGameAchievementUnlocked::new(
                        self.achievement.name,
                        self.game.id,
                        self.achievement.display_name,
                        self.achievement.description.unwrap_or("".to_string()),
                        image,
                        unlocked_date,
                    ),
                ))
            }
            None => Ok(SteamGameAchievement::Locked(
                SteamGameAchievementLocked::new(
                    self.achievement.name,
                    self.game.id,
                    self.achievement.display_name,
                ),
            )),
        }
    }
}
