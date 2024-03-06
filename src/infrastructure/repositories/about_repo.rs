use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

use crate::{prelude::*, infrastructure::{cache::Cache, config::Config}, domain::models::about::About};

const FILE_NAME: &str = "about.md";

#[derive(Debug, Clone)]
pub struct AboutRepo {
    about_text: Arc<RwLock<String>>,
}

impl AboutRepo {
    pub fn new() -> Self {
        Self { about_text: Arc::new(RwLock::new(String::new())) }
    }

    pub fn from_archive(archive: AboutRepoArchive) -> Self {
        Self { about_text: Arc::new(RwLock::new(archive.about_text)) }
    }

    pub async fn reload(&self, config: &Config, cache: &Cache) -> Result<()> {
        if let Some(about_text) = cache.read_cached_file(FILE_NAME, config).await? {
            let mut about_text_ref = self.about_text.write().await;

            *about_text_ref = about_text;
        }
        Ok(())
    }

    pub async fn get_archived(&self) -> Result<AboutRepoArchive> {
        let about_text = self.about_text.read().await;

        Ok(AboutRepoArchive {
            about_text: about_text.clone(),
        })
    }

    pub async fn get_about(&self) -> About {
        let about_text = self.about_text.read().await;

        About::new(about_text.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AboutRepoArchive {
    about_text: String,
}