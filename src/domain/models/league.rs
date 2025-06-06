use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{repositories::LeagueRepo, state::State},
    prelude::Result,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueChampNote {
    pub name: String,
    pub role: String,
    pub lane: String,
    pub q: String,
    pub w: String,
    pub e: String,
    pub r: String,
}

#[derive(Debug, Clone)]
pub struct LeagueGameStats {
    pub playtime: Duration,
    pub last_played: DateTime<Utc>,
}

pub struct LeaugeData {
    pub champ_notes: Vec<LeagueChampNote>,
}

impl LeaugeData {
    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            champ_notes: state.league_repo().find_all_champ_notes_by_name().await?,
        })
    }
}
