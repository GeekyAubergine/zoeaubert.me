use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    infrastructure::{
        cache::Cache,
        config::Config,
        content_dir::{self, ContentDir},
    },
    prelude::*,
};

const FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Clone)]
pub struct SillyNamesRepo {
    silly_names: Arc<RwLock<Vec<String>>>,
}

impl SillyNamesRepo {
    pub fn new() -> Self {
        Self {
            silly_names: Arc::new(RwLock::new(vec!["Zoe Aubert".to_owned()])),
        }
    }

    pub async fn commit(&self, silly_names: Vec<String>) {
        let mut silly_names_ref = self.silly_names.write().await;
        *silly_names_ref = silly_names.clone();
    }

    pub async fn get(&self) -> Vec<String> {
        self.silly_names.read().await.clone()
    }
}
