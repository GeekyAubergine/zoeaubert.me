use crate::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::{repositories::ReferralsRepo, state::State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Referral {
    pub name: String,
    pub description: String,
    pub url: Url,
}

impl Referral {
    pub fn new(name: String, description: String, url: Url) -> Self {
        Self {
            name,
            description,
            url,
        }
    }
}

pub struct Referrals {
    pub referrals: Vec<Referral>,
}

impl Referrals {
    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            referrals: state.referrals_repo().find_all().await?,
        })
    }
}
