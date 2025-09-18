use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use crate::prelude::*;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::services::file_service::{ArchiveFile, FileService, ReadableFile, WritableFile};

const FILE_NAME: &str = "processing_cache_service.json";

// All are 1 minute less than the actual period to account for time drift
pub const FIFTEEN_MINUTES_PERIOD: Duration = Duration::new(15 * 60 - 60, 0);
pub const ONE_HOUR_PERIOD: Duration = Duration::new(60 * 60 - 60, 0);
pub const ONE_DAY_PERIOD: Duration = Duration::new(60 * 60 * 24 - 60, 0);

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct Hashed<'l> {
    key: String,
    hash: u64,
    pub already_processed: bool,
    parent: &'l ContentHashService,
}

impl<'l> Hashed<'l> {
    pub fn set_processed(self) -> Result<()> {
        self.parent.set_processed(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HashCache {
    hashes: HashMap<String, u64>,
}

pub struct ContentHashService {
    file: ArchiveFile,
    data: Arc<RwLock<HashCache>>,
}

impl ContentHashService {
    pub fn new() -> Result<Self> {
        let file = FileService::archive(PathBuf::from(FILE_NAME));
        let data = file.read_json_or_default()?;

        Ok(Self {
            file,
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub fn hash<D: Hash>(&self, key: String, hashable: &D) -> Hashed {
        let data = self.data.read().unwrap();

        let hash = calculate_hash(&hashable);

        let already_processed = match data.hashes.get(&key) {
            Some(stored_hash) => stored_hash == &hash,
            None => false,
        };

        Hashed {
            key,
            hash,
            already_processed,
            parent: self,
        }
    }

    pub fn set_processed(&self, hashed: Hashed) -> Result<()> {
        let mut data = self.data.write().unwrap();

        data.hashes.insert(hashed.key, hashed.hash);

        self.file.write_json(&*data)?;

        Ok(())
    }
}
