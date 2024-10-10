use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{domain::repositories::SillyNamesRepo, prelude::*};
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SillyNamesRepoData {
    silly_names: Vec<String>,
}

pub struct SillyNamesRepoMemory {
    data: Arc<RwLock<SillyNamesRepoData>>,
}

impl SillyNamesRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(SillyNamesRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl SillyNamesRepo for SillyNamesRepoMemory {
    async fn find_all(&self) -> Result<Vec<String>> {
        let data = self.data.read().await;

        Ok(data.silly_names.clone())
    }

    async fn commit(&self, names: Vec<String>) -> Result<()> {
        let mut data = self.data.write().await;
        data.silly_names = names;

        Ok(())
    }
}
