use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::models::referral::Referral;
use crate::domain::repositories::{AboutTextRepo, ReferralsRepo};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ReferralsRepoData {
    referrals: Vec<Referral>,
}

pub struct ReferralsRepoMemory {
    data: Arc<RwLock<ReferralsRepoData>>,
}

impl ReferralsRepoMemory {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(ReferralsRepoData::default())),
        }
    }
}

#[async_trait::async_trait]
impl ReferralsRepo for ReferralsRepoMemory {
    async fn find_all(&self) -> Result<Vec<Referral>> {
        let data = self.data.read().await;
        Ok(data.referrals.clone())
    }

    async fn commit(&self, referrals: Vec<Referral>) -> Result<()> {
        let mut data = self.data.write().await;
        data.referrals = referrals;
        Ok(())
    }
}
