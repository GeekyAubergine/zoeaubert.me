use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{prelude::*, infrastructure::repositories::{games_repo::{SteamOwnedGame, GamesRepo, GameRepoArchive}, about_repo::AboutRepoArchive, faq_repo::FaqRepoArchive, lego_repo::{LegoRepo, LegoRepoArchive}}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archive {
    about: Option<AboutRepoArchive>,
    faq: Option<FaqRepoArchive>,
    games: Option<GameRepoArchive>,
    lego: Option<LegoRepoArchive>,
    last_updated: DateTime<Utc>,
}
