use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::repositories::AboutTextRepo;

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AboutTextRepoData {
    short: String,
    long: String,
}

impl AboutTextRepoData {
    pub fn new(short: String, long: String) -> Self {
        Self { short, long }
    }
}

pub struct AboutTextRepoMemory {
    data: Arc<RwLock<AboutTextRepoData>>,
}

impl AboutTextRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(AboutTextRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl AboutTextRepo for AboutTextRepoMemory {
    async fn find_short(&self) -> Result<String> {
        let data = self.data.read().await;
        Ok(data.short.clone())
    }

    async fn find_long(&self) -> Result<String> {
        let data = self.data.read().await;
        Ok(data.long.clone())
    }

    async fn commit(&self, short: String, long: String) -> Result<()> {
        let mut data = self.data.write().await;
        data.short = short;
        data.long = long;
        Ok(())
    }
}
