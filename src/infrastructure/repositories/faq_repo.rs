use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;

use crate::{prelude::*, infrastructure::{cache::Cache, config::Config}, domain::models::{about::About, faq::Faq}};

const FILE_NAME: &str = "about.md";

#[derive(Debug, Clone)]
pub struct FaqRepo {
    faq_text: Arc<RwLock<String>>,
}

impl FaqRepo {
    pub fn new() -> Self {
        Self { faq_text: Arc::new(RwLock::new(String::new())) }
    }

    pub fn from_archive(archive: FaqRepoArchive) -> Self {
        Self { faq_text: Arc::new(RwLock::new(archive.faq_text)) }
    }

    pub async fn reload(&self, config: &Config, cache: &Cache) -> Result<()> {
        if let Some(faq_text) = cache.read_cached_file(FILE_NAME, config).await? {
            let mut faw_text_ref = self.faq_text.write().await;

            *faw_text_ref = faq_text;
        }
        Ok(())
    }

    pub async fn get_archived(&self) -> Result<FaqRepoArchive> {
        let faq_text = self.faq_text.read().await;

        Ok(FaqRepoArchive {
            faq_text: faq_text.clone(),
        })
    }

    pub async fn get_faq(&self) -> Faq {
        let faq_text = self.faq_text.read().await;

        Faq::new(faq_text.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqRepoArchive {
    faq_text: String,
}