use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::status_lol;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Post {
    #[serde(rename = "status_lol")]
    StatusLol(status_lol::StatusLolPost),
}

impl Post {
    pub fn key(&self) -> &str {
        match self {
            Self::StatusLol(status_lol) => status_lol.key(),
        }
    }

    pub fn permalink(&self) -> &str {
        match self {
            Self::StatusLol(status_lol) => status_lol.permalink(),
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::StatusLol(status_lol) => status_lol.date(),
        }
    }
}