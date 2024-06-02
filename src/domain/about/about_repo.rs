use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    infrastructure::{
        cache::Cache,
        config::Config,
        content_dir::{self, ContentDir},
    },
    prelude::*,
};

use super::about_models::About;

const FILE_NAME_SHORT: &str = "about_short.md";
const FILE_NAME_LONG: &str = "about_long.md";

#[derive(Debug, Clone)]
pub struct AboutRepo {
    about: Arc<RwLock<About>>,
}

impl AboutRepo {
    pub fn new() -> Self {
        Self {
            about: Arc::new(RwLock::new(About::default())),
        }
    }

    pub async fn reload(&self, config: &Config, content_dir: &ContentDir) -> Result<()> {
        if let Some(short_about) = content_dir.read_file(FILE_NAME_SHORT, config).await? {
            if let Some(long_about) = content_dir.read_file(FILE_NAME_LONG, config).await? {
                let mut about_ref = self.about.write().await;

                *about_ref = About::new(short_about, long_about);
            }
        }
        Ok(())
    }

    pub async fn get(&self) -> About {
        self.about.read().await.clone()
    }
}
