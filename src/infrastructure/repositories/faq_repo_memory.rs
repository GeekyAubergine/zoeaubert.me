use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::repositories::{AboutTextRepo, FaqRepo};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct FaqRepoData {
    faq: String,
}

pub struct FaqRepoMemory {
    data: Arc<RwLock<FaqRepoData>>,
}

impl FaqRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(FaqRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl FaqRepo for FaqRepoMemory {
    async fn find(&self) -> Result<String> {
        let data = self.data.read().await;

        Ok(data.faq.clone())
    }

    async fn commit(&self, faq: String) -> Result<()> {
        let mut data = self.data.write().await;
        data.faq = faq;

        Ok(())
    }
}
