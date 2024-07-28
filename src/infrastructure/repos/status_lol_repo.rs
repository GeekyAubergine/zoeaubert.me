use std::{
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, QueryBuilder};
use tokio::sync::RwLock;

use crate::{
    domain::models::status_lol::StatusLolPost,
    error::DatabaseError,
    get_json,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
    ONE_HOUR_CACHE_PERIOD,
};

#[derive(Debug, Clone)]
pub struct StatusLolRepo {
    database_connection: DatabaseConnection,
}

impl StatusLolRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn commit(&self, posts: Vec<StatusLolPost>) -> Result<()> {
        for post in posts {
            sqlx::query!(
                "
                INSERT INTO status_lol_posts (id, date, content, emoji, original_url)
                VALUES ($1, $2, $3, $4, $5)
                ",
                post.id(),
                post.date(),
                post.content(),
                post.emoji(),
                post.original_url()
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;
        }

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<StatusLolPost>> {
        let rows = sqlx::query!(
            "
            SELECT id, date, content, emoji, original_url
            FROM status_lol_posts
            ORDER BY date DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let posts = rows
            .into_iter()
            .map(|row| {
                StatusLolPost::new(row.id, row.date, row.content, row.emoji, row.original_url)
            })
            .collect();

        Ok(posts)
    }
}
