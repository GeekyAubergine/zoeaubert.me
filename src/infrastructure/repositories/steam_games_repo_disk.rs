use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::repositories::SteamGamesRepo;
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

use crate::domain::models::steam::SteamGame;

const FILE_NAME: &str = "steam_games.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SteamGamesRepoData {
    games: HashMap<u32, SteamGame>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GamesRepoDisk {
    data: Arc<RwLock<SteamGamesRepoData>>,
    file_service: FileServiceDisk,
}

impl GamesRepoDisk {
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
impl SteamGamesRepo for GamesRepoDisk {
    async fn find_by_game_id(&self, game_id: u32) -> Result<Option<SteamGame>> {
        let data = self.data.read().await;

        Ok(data.games.get(&game_id).cloned())
    }

    async fn find_all_games(&self) -> Result<Vec<SteamGame>> {
        let data = self.data.read().await;

        Ok(data.games.values().cloned().collect::<Vec<SteamGame>>())
    }

    async fn find_total_playtime(&self) -> Result<u32> {
        let data = self.data.read().await;

        let total_playtime = data.games.values().map(|game| game.playtime).sum();

        Ok(total_playtime)
    }

    async fn find_total_games(&self) -> Result<u32> {
        let data = self.data.read().await;

        Ok(data.games.len() as u32)
    }

    async fn find_most_recently_updated_at(&self) -> Result<Option<DateTime<Utc>>> {
        let data = self.data.read().await;

        Ok(Some(data.updated_at))
    }

    async fn commit(&self, game: &SteamGame) -> Result<()> {
        let mut data = self.data.write().await;

        data.games.insert(game.id, game.clone());
        data.updated_at = Utc::now();

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
