use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

use crate::{
    domain::{
        models::lego::{LegoMinifig, LegoSet},
        repositories::LegoRepo,
    },
};

const FILE_NAME: &str = "lego.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegoRepoData {
    sets: HashMap<u32, LegoSet>,
    minifigs: HashMap<String, LegoMinifig>,
    last_updated_at: Option<DateTime<Utc>>,
}

pub struct LegoRepoDisk {
    data: Arc<RwLock<LegoRepoData>>,
    file_service: FileServiceDisk,
}

impl LegoRepoDisk {
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
impl LegoRepo for LegoRepoDisk {
    async fn find_all_sets(&self) -> Result<Vec<LegoSet>> {
        let data = self.data.read().await;

        let mut sets = data.sets.values().cloned().collect::<Vec<LegoSet>>();

        sets.sort_by(|a, b| b.pieces.cmp(&a.pieces));

        Ok(sets)
    }

    async fn find_all_minifigs(&self) -> Result<Vec<LegoMinifig>> {
        let data = self.data.read().await;

        let mut minifigs = data
            .minifigs
            .values()
            .cloned()
            .collect::<Vec<LegoMinifig>>();

        minifigs.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(minifigs)
    }

    async fn find_total_pieces(&self) -> Result<u32> {
        let data = self.data.read().await;

        Ok(data.sets.values().map(|set| set.pieces).sum())
    }

    async fn find_total_sets(&self) -> Result<u32> {
        let data = self.data.read().await;

        Ok(data.sets.len() as u32)
    }

    async fn find_total_minifigs(&self) -> Result<u32> {
        let data = self.data.read().await;

        Ok(data.minifigs.len() as u32)
    }

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>> {
        let data = self.data.read().await;

        Ok(data.last_updated_at)
    }

    async fn commit_set(&self, set: &LegoSet) -> Result<()> {
        let mut data = self.data.write().await;

        data.sets.insert(set.id, set.clone());
        data.last_updated_at = Some(Utc::now());

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }

    async fn commit_minifig(&self, minifig: &LegoMinifig) -> Result<()> {
        let mut data = self.data.write().await;

        data.minifigs.insert(minifig.id.clone(), minifig.clone());
        data.last_updated_at = Some(Utc::now());

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
