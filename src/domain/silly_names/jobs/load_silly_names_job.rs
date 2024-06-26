use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    domain::silly_names,
    infrastructure::{
        app_state::AppState,
        bus::job_runner::Job,
        cache::Cache,
        config::Config,
        content_dir::{self, ContentDir},
    },
    prelude::*,
};

const FILE_NAME: &str = "silly_names.csv";
pub struct LoadSillyNamesJob;

impl LoadSillyNamesJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadSillyNamesJob {
    fn name(&self) -> &str {
        "LoadSillyNamesJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading silly names");
        let silly_names = app_state
            .content_dir()
            .read_file(FILE_NAME, app_state.config())
            .await?;

        let silly_names = silly_names
            .split('\n')
            .filter_map(|s| Some(s.trim().split(',').next()?.to_owned()))
            .collect::<Vec<String>>();

        app_state.silly_names_repo().commit(silly_names).await;

        Ok(())
    }
}
