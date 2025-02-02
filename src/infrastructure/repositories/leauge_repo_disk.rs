use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Datelike;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::album::{Album, AlbumPhoto};
use crate::domain::models::league::LeagueChampNote;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::LeagueRepo;
use crate::domain::services::FileService;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

pub struct LeagueRepoDisk {
    champ_notes: Arc<RwLock<Vec<LeagueChampNote>>>,
}

impl LeagueRepoDisk {
    pub async fn new() -> Result<Self> {
        let champ_notes = Arc::new(RwLock::new(Vec::new()));

        Ok(Self { champ_notes })
    }
}

#[async_trait::async_trait]
impl LeagueRepo for LeagueRepoDisk {
    async fn find_all_champ_notes_by_name(&self) -> Result<Vec<LeagueChampNote>> {
        let mut champ_notes = self.champ_notes.read().await.clone();

        champ_notes.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(champ_notes)
    }

    async fn commit_champ_notes(&self, champ_notes: Vec<LeagueChampNote>) -> Result<()> {
        let mut data = self.champ_notes.write().await;
        *data = champ_notes.clone();

        Ok(())
    }
}
