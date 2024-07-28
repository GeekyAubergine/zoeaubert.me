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
    domain::models::game::Game,
    error::DatabaseError,
    get_json,
    infrastructure::{
        app_state::{AppState, DatabaseConnection},
        config::Config,
    },
    prelude::*,
    ONE_DAY_CACHE_PERIOD, ONE_HOUR_CACHE_PERIOD,
};

use super::game_achievements_repo::GameAchievementsRepo;

struct SelectedRow {
    id: i64,
    name: String,
    header_image_url: String,
    playtime: i64,
    last_played: DateTime<Utc>,
    link_url: String,
}

impl From<SelectedRow> for Game {
    fn from(row: SelectedRow) -> Self {
        Game::new(
            row.id as u32,
            row.name,
            row.header_image_url,
            row.playtime as u32,
            row.last_played,
            row.link_url,
        )
    }
}

#[derive(Debug, Clone)]
pub struct GamesRepo {
    database_connection: DatabaseConnection,
}

impl GamesRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn commit(&self, game: &Game) -> Result<()> {
        if let Some(_) = self.find_by_id(game.id()).await? {
            sqlx::query!(
                "
                UPDATE games
                SET name = $2, header_image_url = $3, playtime = $4, last_played = $5, link_url = $6
                WHERE id = $1
                ",
                game.id() as i64,
                game.name(),
                game.header_image_url(),
                game.playtime() as i64,
                game.last_played(),
                game.link_url()
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        sqlx::query!(
            "
            INSERT INTO games (id, name, header_image_url, playtime, last_played, link_url)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
            game.id() as i64,
            game.name(),
            game.header_image_url(),
            game.playtime() as i64,
            game.last_played(),
            game.link_url()
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: u32) -> Result<Option<Game>> {
        let row = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, header_image_url, playtime, last_played, link_url
            FROM games
            WHERE id = $1
            ",
            id as i64
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        match row {
            Some(row) => Ok(Some(row.into())),
            None => Ok(None),
        }
    }

    pub async fn find_all_games(&self) -> Result<Vec<Game>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, header_image_url, playtime, last_played, link_url
            FROM games
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let games = rows.into_iter().map(Game::from).collect();

        Ok(games)
    }

    pub async fn get_games_by_most_recently_played(&self) -> Result<Vec<Game>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, header_image_url, playtime, last_played, link_url
            FROM games
            ORDER BY last_played DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let games = rows.into_iter().map(Game::from).collect();

        Ok(games)
    }

    pub async fn get_games_by_most_played(&self) -> Result<Vec<Game>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, header_image_url, playtime, last_played, link_url
            FROM games
            ORDER BY playtime DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let games = rows.into_iter().map(Game::from).collect();

        Ok(games)
    }

    pub async fn get_total_play_time(&self) -> Result<u32> {
        let row = sqlx::query!(
            "
            SELECT SUM(playtime) as total_playtime
            FROM games
            "
        )
        .fetch_one(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        match row.total_playtime {
            Some(playtime) => Ok(playtime as u32),
            None => Ok(0),
        }
    }

    pub async fn get_total_play_time_hours(&self) -> Result<f32> {
        self.get_total_play_time()
            .await
            .map(|playtime| playtime as f32 / 60.0)
    }
}
