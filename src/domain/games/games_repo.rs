use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
    vec,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinSet};

use crate::{
    get_json, infrastructure::config::Config, prelude::*, ONE_DAY_CACHE_PERIOD,
    ONE_HOUR_CACHE_PERIOD,
};

use super::games_models::{Game, GameAchievement, GameAchievementLocked, GameAchievementUnlocked};

#[derive(Debug, Clone, Default)]
pub struct GamesRepo {
    games: Arc<RwLock<HashMap<u32, Game>>>,
    last_updated: Arc<RwLock<DateTime<Utc>>>,
}

impl GamesRepo {
    pub async fn rebuild_from_archive(&self, archive: GameRepoArchive) {
        let mut games = self.games.write().await;
        let mut last_updated = self.last_updated.write().await;

        *games = archive.games;
        *last_updated = archive.last_updated;
    }

    pub async fn commit(&self, game: Game) {
        let mut games = self.games.write().await;
        games.insert(game.id(), game);

        let mut last_updated = self.last_updated.write().await;
        *last_updated = Utc::now();
    }

    pub async fn get_last_updated(&self) -> DateTime<Utc> {
        *self.last_updated.read().await
    }

    pub async fn get_archived(&self) -> GameRepoArchive {
        let games = self.games.read().await;

        GameRepoArchive {
            games: games.clone(),
            last_updated: *self.last_updated.read().await,
        }
    }

    pub async fn get_game(&self, id: u32) -> Option<Game> {
        let games = self.games.read().await;

        games.get(&id).cloned()
    }

    pub async fn get_all_games(&self) -> HashMap<u32, Game> {
        let games = self.games.read().await;

        games
            .iter()
            .map(|(key, game)| (*key, game.clone()))
            .collect()
    }

    pub async fn get_games_by_most_recently_played(&self) -> Vec<Game> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by(|a, b| b.last_played().cmp(a.last_played()));

        games_array
    }

    pub async fn get_games_by_most_played(&self) -> Vec<Game> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by_key(|b| std::cmp::Reverse(b.playtime()));

        games_array
    }

    pub async fn get_games_by_most_completed_achievements(&self) -> Vec<Game> {
        let games = self.games.read().await;

        let mut games_array = games.values().cloned().collect::<Vec<Game>>();

        games_array.sort_by(|a, b| {
            b.achievements_unlocked_count()
                .cmp(&a.achievements_unlocked_count())
        });

        games_array
    }

    pub async fn get_total_play_time(&self) -> u32 {
        let games = self.games.read().await;

        games.values().map(|game| game.playtime()).sum()
    }

    pub async fn get_total_play_time_hours(&self) -> f32 {
        let total_playtime = self.get_total_play_time().await;

        total_playtime as f32 / 60.0
    }

    pub async fn get_all_unlocked_acheivements_for_game(&self, id: u32) -> Vec<GameAchievementUnlocked> {
        let games = self.games.read().await;

        match games.get(&id) {
            Some(game) => game
                .achievements()
                .values()
                .filter_map(|achievement| match achievement {
                    GameAchievement::Unlocked(unlocked) => Some(unlocked.clone()),
                    _ => None,
                })
                .collect(),
            None => vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRepoArchive {
    games: HashMap<u32, Game>,
    last_updated: DateTime<Utc>,
}
