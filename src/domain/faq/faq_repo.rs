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

use super::faq_models::Faq;

const FILE_NAME: &str = "faq.md";

#[derive(Debug, Clone)]
pub struct FaqRepo {
    faq_text: Arc<RwLock<Faq>>,
}

impl FaqRepo {
    pub fn new() -> Self {
        Self {
            faq_text: Arc::new(RwLock::new(Faq::default())),
        }
    }

    pub async fn reload(&self, config: &Config, content_dir: &ContentDir) -> Result<()> {
        if let Some(faq_text) = content_dir.read_file(FILE_NAME, config).await? {
            let mut faq_ref = self.faq_text.write().await;

            *faq_ref = Faq::new(faq_text);
        }
        Ok(())
    }
    pub async fn get(&self) -> Faq {
        self.faq_text.read().await.clone()
    }
}
