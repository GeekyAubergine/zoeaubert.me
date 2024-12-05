use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::steam::{
    SteamGameAchievement, SteamGameAchievementLocked, SteamGameAchievementUnlocked,
};
use crate::domain::repositories::{SteamAchievementsRepo, SteamGamesRepo};
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

use crate::domain::models::steam::SteamGame;

const FILE_NAME: &str = "steam_achievements.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SteamAchievementsForGame {
    game_id: u32,
    locked_achievements: HashMap<String, SteamGameAchievementLocked>,
    unlocked_achievements: HashMap<String, SteamGameAchievementUnlocked>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct GameAchievementsRepoData {
    data: HashMap<u32, SteamAchievementsForGame>,
}

pub struct GameAchievementsRepoDisk {
    data: Arc<RwLock<GameAchievementsRepoData>>,
    file_service: FileServiceDisk,
}

impl GameAchievementsRepoDisk {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl SteamAchievementsRepo for GameAchievementsRepoDisk {
    async fn find_all_unlocked_by_unlocked_date(
        &self,
        game_id: u32,
    ) -> Result<Vec<SteamGameAchievementUnlocked>> {
        let data = self.data.read().await;

        let game_achievements = match data.data.get(&game_id) {
            Some(game_achievements) => game_achievements,
            None => return Ok(vec![]),
        };

        let mut unlocked_achievements = game_achievements
            .unlocked_achievements
            .values()
            .cloned()
            .collect::<Vec<SteamGameAchievementUnlocked>>();

        unlocked_achievements.sort_by(|a, b| b.unlocked_date.cmp(&a.unlocked_date));

        Ok(unlocked_achievements)
    }

    async fn find_all_locked_by_name(&self, game_id: u32) -> Result<Vec<SteamGameAchievementLocked>> {
        let data = self.data.read().await;

        let game_achievements = match data.data.get(&game_id) {
            Some(game_achievements) => game_achievements,
            None => return Ok(vec![]),
        };
        let mut locked_achievements = game_achievements
            .locked_achievements
            .values()
            .cloned()
            .collect::<Vec<SteamGameAchievementLocked>>();

        locked_achievements.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        Ok(locked_achievements)
    }

    async fn commit(&self, game: &SteamGame, achievement: &SteamGameAchievement) -> Result<()> {
        let mut data = self.data.write().await;

        match achievement {
            SteamGameAchievement::Locked(locked) => {
                let game_achievements =
                    data.data
                        .entry(game.id)
                        .or_insert_with(|| SteamAchievementsForGame {
                            game_id: game.id,
                            locked_achievements: HashMap::new(),
                            unlocked_achievements: HashMap::new(),
                        });

                game_achievements
                    .locked_achievements
                    .insert(locked.id.clone(), locked.clone());
            }
            SteamGameAchievement::Unlocked(unlocked) => {
                let game_achievements =
                    data.data
                        .entry(game.id)
                        .or_insert_with(|| SteamAchievementsForGame {
                            game_id: game.id,
                            locked_achievements: HashMap::new(),
                            unlocked_achievements: HashMap::new(),
                        });

                game_achievements
                    .unlocked_achievements
                    .insert(unlocked.id.clone(), unlocked.clone());
            }
        };

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
