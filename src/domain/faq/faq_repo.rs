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

#[derive(Debug, Clone, Default)]
pub struct FaqRepo {
    faq_text: Arc<RwLock<Faq>>,
}

impl FaqRepo {
    pub async fn commit(&self, faq: Faq) {
        *self.faq_text.write().await = faq;
    }

    pub async fn get(&self) -> Faq {
        self.faq_text.read().await.clone()
    }
}
