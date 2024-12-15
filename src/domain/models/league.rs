use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
