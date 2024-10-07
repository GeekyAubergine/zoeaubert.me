use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
    vec,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinSet};
use tracing::info;

use crate::{
    domain::models::game_achievement::{
        GameAchievement, GameAchievementLocked, GameAchievementUnlocked, GameAchievements,
    },
    error::DatabaseError,
    get_json,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
    ONE_DAY_CACHE_PERIOD, ONE_HOUR_CACHE_PERIOD,
};

struct SelectedRow {
    id: String,
    game_id: i64,
    display_name: String,
    description: String,
    locked_image_url: Option<String>,
    unlocked_image_url: Option<String>,
    unlocked_date: Option<DateTime<Utc>>,
    global_unlocked_percentage: f64,
    updated_at: DateTime<Utc>,
}

impl From<SelectedRow> for GameAchievement {
    fn from(row: SelectedRow) -> Self {
        if let Some(unlocked_date) = row.unlocked_date {
            GameAchievement::Unlocked(GameAchievementUnlocked::new(
                row.id,
                row.game_id as u32,
                row.display_name,
                row.description,
                row.unlocked_image_url.unwrap_or("".to_string()),
                unlocked_date,
                row.global_unlocked_percentage as f32,
                row.updated_at,
            ))
        } else {
            GameAchievement::Locked(GameAchievementLocked::new(
                row.id,
                row.game_id as u32,
                row.display_name,
                row.description,
                row.locked_image_url.unwrap_or("".to_string()),
                row.global_unlocked_percentage as f32,
                row.updated_at,
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameAchievementsRepo {
    database_connection: DatabaseConnection,
}

impl GameAchievementsRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_by_id(&self, id: &str, game_id: u32) -> Result<GameAchievement> {
        let row = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, game_id, display_name, description, locked_image_url, unlocked_image_url, unlocked_date, global_unlocked_percentage, updated_at
            FROM game_achievements
            WHERE id = $1 AND game_id = $2
            ",
            id,
            game_id as i64
        )
        .fetch_one(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(GameAchievement::from(row))
    }

    pub async fn find_by_game_id(&self, game_id: u32) -> Result<GameAchievements> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, game_id, display_name, description, locked_image_url, unlocked_image_url, unlocked_date, global_unlocked_percentage, updated_at
            FROM game_achievements
            WHERE game_id = $1
            ",
            game_id as i64
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let achievements = rows.into_iter().map(GameAchievement::from).collect();

        let achievements = GameAchievements::from_achievements(game_id, achievements);

        Ok(achievements)
    }

    pub async fn find_all_unlocked_for_game_id(
        &self,
        game_id: u32,
    ) -> Result<Vec<GameAchievementUnlocked>> {
        let rows = sqlx::query!(
            "
            SELECT id, game_id, display_name, description, unlocked_image_url, unlocked_date, global_unlocked_percentage, updated_at
            FROM game_achievements
            WHERE game_id = $1 AND unlocked_date IS NOT NULL
            ",
            game_id as i64
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let achievements = rows
            .into_iter()
            .map(|row| {
                GameAchievementUnlocked::new(
                    row.id,
                    row.game_id as u32,
                    row.display_name,
                    row.description,
                    row.unlocked_image_url.unwrap_or("".to_string()),
                    row.unlocked_date.unwrap(),
                    row.global_unlocked_percentage as f32,
                    row.updated_at,
                )
            })
            .collect();

        Ok(achievements)
    }

    pub async fn commit(&self, achievment: &GameAchievement) -> Result<()> {
        info!(
            "Updating or commiting {} {}",
            achievment.id(),
            achievment.game_id()
        );
        if let Ok(_) = self.find_by_id(achievment.id(), achievment.game_id()).await {
            match achievment {
                GameAchievement::Locked(achievement) => {
                    sqlx::query!(
                        "
                        UPDATE game_achievements
                        SET display_name = $2, description = $3, locked_image_url = $4, unlocked_image_url = NULL, unlocked_date = NULL, global_unlocked_percentage = $5, updated_at = NOW()
                        WHERE id = $1 and game_id = $6
                        ",
                        achievement.id(),
                        achievement.display_name(),
                        achievement.description(),
                        achievement.image_url(),
                        achievement.global_unlocked_percentage(),
                        achievement.game_id() as i64
                    )
                    .execute(&self.database_connection)
                    .await
                    .map_err(DatabaseError::from_query_error)?;
                }
                GameAchievement::Unlocked(achievement) => {
                    sqlx::query!(
                        "
                        UPDATE game_achievements
                        SET display_name = $2, description = $3, locked_image_url = NULL, unlocked_image_url = $4, unlocked_date = $5, global_unlocked_percentage = $6, updated_at = NOW()
                        WHERE id = $1 and game_id = $7
                        ",
                        achievement.id(),
                        achievement.display_name(),
                        achievement.description(),
                        achievement.image_url(),
                        achievement.unlocked_date(),
                        achievement.global_unlocked_percentage(),
                        achievement.game_id() as i64
                    )
                    .execute(&self.database_connection)
                    .await
                    .map_err(DatabaseError::from_query_error)?;
                }
            }

            return Ok(());
        }

        match achievment {
            GameAchievement::Locked(achievement) => {
                sqlx::query!(
                    "
                    INSERT INTO game_achievements (id, game_id, display_name, description, locked_image_url, global_unlocked_percentage, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, NOW())
                    ",
                    achievement.id(),
                    achievement.game_id() as i64,
                    achievement.display_name(),
                    achievement.description(),
                    achievement.image_url(),
                    achievement.global_unlocked_percentage()
                )
                .execute(&self.database_connection)
                .await
                .map_err(DatabaseError::from_query_error)?;

                Ok(())
            }
            GameAchievement::Unlocked(achievement) => {
                sqlx::query!(
                    "
                    INSERT INTO game_achievements (id, game_id, display_name, description,  unlocked_image_url, unlocked_date, global_unlocked_percentage, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
                    ",
                    achievement.id(),
                    achievement.game_id() as i64,
                    achievement.display_name(),
                    achievement.description(),
                    achievement.image_url(),
                    achievement.unlocked_date(),
                    achievement.global_unlocked_percentage()
                )
                .execute(&self.database_connection)
                .await
                .map_err(DatabaseError::from_query_error)?;

                Ok(())
            }
        }
    }
}
