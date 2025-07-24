use std::path::{Path, PathBuf};
use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::prelude::*;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::services::file_service::{File, FilePath};

const FILE_NAME: &str = "query_limiting_service.json";

// All are 1 minute less than the actual period to account for time drift
pub const FIFTEEN_MINUTES_PERIOD: Duration = Duration::new(15 * 60 - 60, 0);
pub const ONE_HOUR_PERIOD: Duration = Duration::new(60 * 60 - 60, 0);
pub const ONE_DAY_PERIOD: Duration = Duration::new(60 * 60 * 24 - 60, 0);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryLimitingData {
    queries: HashMap<String, DateTime<Utc>>,
}

pub struct QueryLimitingService2 {
    data: Arc<RwLock<QueryLimitingData>>,
}

impl QueryLimitingService2 {
    pub async fn new() -> Result<Self> {
        let data = File::from_path(FilePath::archive(FILE_NAME))
            .read_as_json_or_default()
            .await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub async fn can_query(&self, query: &str, no_query_duration: &Duration) -> Result<bool> {
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

            File::from_path(FilePath::archive(FILE_NAME)).write_json(&*data).await?;
        }

        Ok(can_query)
    }

    pub async fn can_query_within_fifteen_minutes(&self, query: &str) -> Result<bool> {
        self.can_query(query, &FIFTEEN_MINUTES_PERIOD).await
    }

    pub async fn can_query_within_hour(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_HOUR_PERIOD).await
    }

    pub async fn can_query_within_day(&self, query: &str) -> Result<bool> {
        self.can_query(query, &ONE_DAY_PERIOD).await
    }
}
