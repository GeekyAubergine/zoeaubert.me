use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum BattleReportGame {
    FortyK,
    AgeOfSigmar,
    KillTeam,
    LegionsImperialis,
}

#[derive(Debug, Clone)]
pub struct BattleReport {
    pub game: BattleReportGame,
    pub date: DateTime<Utc>,
    pub myArmy: String,
    pub opponent: String,
}
