use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use crate::services::file_service::{ArchiveFile, FileService, ReadableFile, WritableFile};

const FILE_NAME: &str = "query_limiting_service.json";

// All are 1 minute less than the actual period to account for time drift
pub const ONE_HOUR_PERIOD: Duration = Duration::new(60 * 60 - 60, 0);
pub const ONE_DAY_PERIOD: Duration = Duration::new(60 * 60 * 24 - 60, 0);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryLimitingData {
    queries: DashMap<String, DateTime<Utc>>,
}

#[derive(Debug)]
pub struct QueryLimitingService {
    file: ArchiveFile,
    data: QueryLimitingData,
}

impl QueryLimitingService {
    pub fn new() -> Result<Self> {
        let file = FileService::archive(PathBuf::from(FILE_NAME));
        let data = file.read_json_or_default()?;

        Ok(Self { file, data })
    }

    pub fn can_query(&self, query: &str, no_query_duration: &Duration) -> Result<bool> {
        let can_query = match self.data.queries.get(query) {
            Some(last_queried) => *last_queried + *no_query_duration <= Utc::now(),
            None => true,
        };

        if can_query {
            self.data.queries.insert(query.to_string(), Utc::now());

            self.file.write_json(&self.data)?;
        }

        Ok(can_query)
    }

    pub fn can_query_within_hour(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_HOUR_PERIOD)
    }

    pub fn can_query_within_day(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_DAY_PERIOD)
    }
}
