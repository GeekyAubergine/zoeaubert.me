use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{domain::models::lego::{LegoMinifig, LegoSet}, get_json, infrastructure::config::Config, prelude::*, ONE_HOUR_CACHE_PERIOD};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

#[derive(Debug, Clone, Default)]
pub struct LegoRepo {
    sets: Arc<RwLock<HashMap<String, LegoSet>>>,
    minifigs: Arc<RwLock<HashMap<String, LegoMinifig>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl LegoRepo {
    pub async fn load_from_archive(&self, archive: LegoRepoArchive) {
        let mut sets_ref = self.sets.write().await;

        *sets_ref = archive.sets;

        let mut minifigs_ref = self.minifigs.write().await;

        *minifigs_ref = archive.minifigs;

        let mut last_updated = self.last_updated.write().await;

        *last_updated = archive.last_updated;
    }

    pub async fn commit(
        &self,
        lego_sets: HashMap<String, LegoSet>,
        lego_minifigs: HashMap<String, LegoMinifig>,
    ) {
        let mut sets_ref = self.sets.write().await;
        sets_ref.clone_from(&lego_sets);

        let mut minifigs_ref = self.minifigs.write().await;
        minifigs_ref.clone_from(&lego_minifigs);

        let mut last_updated = self.last_updated.write().await;
        *last_updated = Utc::now();
    }

    pub async fn get_last_updated(&self) -> DateTime<Utc> {
        *self.last_updated.read().await
    }

    pub async fn get_archived(&self) -> LegoRepoArchive {
        let sets = self.sets.read().await;
        let minifigs = self.minifigs.read().await;
        let last_updated = *self.last_updated.read().await;

        LegoRepoArchive {
            sets: sets.clone(),
            minifigs: minifigs.clone(),
            last_updated,
        }
    }

    pub async fn get_all_sets(&self) -> HashMap<String, LegoSet> {
        let sets = self.sets.read().await;

        sets.clone()
    }

    pub async fn get_all_minifigs(&self) -> HashMap<String, LegoMinifig> {
        let minifigs = self.minifigs.read().await;

        minifigs.clone()
    }

    pub async fn get_most_piece_sets(&self) -> Vec<u32> {
        let sets = self.sets.read().await;

        let mut sets = sets.values().collect::<Vec<&LegoSet>>();

        sets.sort_by_key(|a| a.pieces());

        sets.iter().map(|set| set.key()).rev().collect()
    }

    pub async fn get_most_owned_minifigs(&self) -> Vec<String> {
        let minifigs = self.minifigs.read().await;

        let mut minifigs = minifigs.values().collect::<Vec<&LegoMinifig>>();

        minifigs.sort_by_key(|a| a.owned_in_sets() + a.owned_loose());

        minifigs
            .iter()
            .map(|minifig| minifig.key())
            .rev()
            .map(|key| key.to_string())
            .collect()
    }

    pub async fn get_total_pieces(&self) -> u32 {
        let sets = self.sets.read().await;

        sets.iter().map(|(_, set)| set.pieces()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoRepoArchive {
    sets: HashMap<String, LegoSet>,
    minifigs: HashMap<String, LegoMinifig>,
    last_updated: DateTime<Utc>,
}
