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

    pub async fn commit(&self, faq: Faq) {
        *self.faq_text.write().await = faq;
    }

    pub async fn get(&self) -> Faq {
        self.faq_text.read().await.clone()
    }
}

impl Default for FaqRepo {
    fn default() -> Self {
        Self::new()
    }
}
