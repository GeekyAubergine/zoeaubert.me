use crate::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

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
