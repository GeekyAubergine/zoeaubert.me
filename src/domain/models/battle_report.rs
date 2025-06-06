use chrono::{DateTime, Utc};

use super::{raw_content::RawContent, image::Image};

#[derive(Debug, Clone)]
pub enum BattleReportGame {
    FortyK,
    AgeOfSigmar,
    KillTeam,
    LegionsImperialis,
}

#[derive(Debug, Clone)]
pub struct BattleReport {
    pub source_content: RawContent,
}
