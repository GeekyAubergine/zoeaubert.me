use chrono::{DateTime, Utc};

use super::{content::Content, image::Image};

#[derive(Debug, Clone)]
pub enum BattleReportGame {
    FortyK,
    AgeOfSigmar,
    KillTeam,
    LegionsImperialis,
}

#[derive(Debug, Clone)]
pub struct BattleReport {
    pub source_content: Content,
}
