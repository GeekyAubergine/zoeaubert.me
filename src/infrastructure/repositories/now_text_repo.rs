use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::repositories::{AboutTextRepo, FaqRepo, NowTextRepo};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct NowTextRepoData {
    now_text: String,
}

pub struct NowTextRepoMemory {
    data: Arc<RwLock<NowTextRepoData>>,
}

impl NowTextRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(NowTextRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl NowTextRepo for NowTextRepoMemory {
    async fn find(&self) -> Result<String> {
        let data = self.data.read().await;

        Ok(data.now_text.clone())
    }

    async fn commit(&self, faq: String) -> Result<()> {
        let mut data = self.data.write().await;
        data.now_text = faq;

        Ok(())
    }
}
