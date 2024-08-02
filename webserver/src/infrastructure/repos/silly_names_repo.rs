use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::{infrastructure::config::Config, prelude::*};

const FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Clone, Default)]
pub struct SillyNamesRepo {
    silly_names: Arc<RwLock<Vec<String>>>,
}

impl SillyNamesRepo {
    pub async fn commit(&self, silly_names: Vec<String>) {
        self.silly_names.write().await.clone_from(&silly_names);
    }

    pub async fn get(&self) -> Vec<String> {
        self.silly_names.read().await.clone()
    }
}
