use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::services::{FileService, QueryLimitingService};
use crate::domain::state::State;
use crate::infrastructure::services::file_service_disk::FileServiceDisk;
use crate::prelude::*;

use crate::domain::{
    models::lego::{LegoMinifig, LegoSet},
    repositories::LegoRepo,
};

const FILE_NAME: &str = "query_limiting_service.json";

// All are 1 minute less than the actual period to account for time drift
pub const FIFTEEN_MINUTES_PERIOD: Duration = Duration::new(15 * 60 - 60, 0);
pub const ONE_HOUR_PERIOD: Duration = Duration::new(60 * 60 - 60, 0);
pub const ONE_DAY_PERIOD: Duration = Duration::new(60 * 60 * 24 - 60, 0);

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegoRepoData {
    queries: HashMap<String, DateTime<Utc>>,
}

pub struct QueryLimitingServiceDisk {
    data: Arc<RwLock<LegoRepoData>>,
    file_service: FileServiceDisk,
}

impl QueryLimitingServiceDisk {
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
impl QueryLimitingService for QueryLimitingServiceDisk {
    async fn can_query(&self, query: &str, no_query_duration: &Duration) -> Result<bool> {
        let mut data = self.data.write().await;

        let can_query = match data.queries.get(query) {
            Some(last_queried) => {
                if *last_queried + *no_query_duration > Utc::now() {
                    false
                } else {
                    true
                }
            }
            None => true,
        };

        if can_query {
            data.queries.insert(query.to_string(), Utc::now());
            self.file_service
                .write_json_file(&make_file_path(&self.file_service), &*data)
                .await?;
        }

        Ok(can_query)
    }

    async fn can_query_within_fifteen_minutes(&self, query: &str) -> Result<bool> {
        self.can_query(query, &FIFTEEN_MINUTES_PERIOD).await
    }

    async fn can_query_within_hour(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_HOUR_PERIOD).await
    }

    async fn can_query_within_day(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_DAY_PERIOD).await
    }
}
