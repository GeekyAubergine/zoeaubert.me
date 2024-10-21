use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::repositories::GamesRepo;
use crate::infrastructure::utils::file_system::write_json_file;
use crate::prelude::*;

use crate::{
    domain::models::games::Game,
    infrastructure::utils::file_system::{make_archive_file_path, read_json_file_or_default},
};

const FILE_NAME: &str = "games.json";

fn make_file_path() -> PathBuf {
    make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GamesRepoData {
    games: HashMap<u32, Game>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GamesRepoDisk {
    data: Arc<RwLock<GamesRepoData>>,
}

impl GamesRepoDisk {
    pub async fn new() -> Result<Self> {
        let data = read_json_file_or_default(&make_file_path()).await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
        })
    }
}

#[async_trait::async_trait]
impl GamesRepo for GamesRepoDisk {
    async fn find_by_game_id(&self, game_id: u32) -> Result<Option<Game>> {
        let data = self.data.read().await;

        Ok(data.games.get(&game_id).cloned())
    }

    async fn find_all_games(&self) -> Result<Vec<Game>> {
        let data = self.data.read().await;

        Ok(data.games.values().cloned().collect::<Vec<Game>>())
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

    async fn commit(&self, game: &Game) -> Result<()> {
        let mut data = self.data.write().await;

        data.games.insert(game.id, game.clone());
        data.updated_at = Utc::now();

        write_json_file(&make_file_path(), &data.clone()).await?;

        Ok(())
    }
}
