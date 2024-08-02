use std::{
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, QueryBuilder};
use tokio::sync::RwLock;

use crate::{
    domain::models::status_lol_post::StatusLolPost,
    error::DatabaseError,
    get_json,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
    ONE_HOUR_CACHE_PERIOD,
};

struct SelectedRow {
    id: String,
    date: DateTime<Utc>,
    content: String,
    emoji: String,
    original_url: String,
    updated_at: DateTime<Utc>,
}

impl From<SelectedRow> for StatusLolPost {
    fn from(row: SelectedRow) -> Self {
        StatusLolPost::new(
            row.id,
            row.date,
            row.content,
            row.emoji,
            row.original_url,
            row.updated_at,
        )
    }
}

#[derive(Debug, Clone)]
pub struct StatusLolPostsRepo {
    database_connection: DatabaseConnection,
}

impl StatusLolPostsRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<StatusLolPost>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, date, content, emoji, original_url, updated_at
            FROM status_lol_posts
            ORDER BY date DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().map(StatusLolPost::from).collect())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<StatusLolPost>> {
        let row = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, date, content, emoji, original_url, updated_at
            FROM status_lol_posts
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row.map(StatusLolPost::from))
    }

    pub async fn find_most_recently_updated_date(&self) -> Result<Option<DateTime<Utc>>> {
        let row = sqlx::query!(
            "
            SELECT updated_at
            FROM status_lol_posts
            ORDER BY updated_at DESC
            LIMIT 1
            "
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row.map(|row| row.updated_at))
    }

    pub async fn commit(&self, posts: Vec<StatusLolPost>) -> Result<()> {
        for post in posts {
            if let Some(_) = self.find_by_id(post.id()).await? {
                sqlx::query!(
                    "
                        UPDATE status_lol_posts
                        SET date = $2, content = $3, emoji = $4, original_url = $5, updated_at = now()
                        WHERE id = $1
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

                continue;
            }

            sqlx::query!(
                "
                    INSERT INTO status_lol_posts (id, date, content, emoji, original_url, updated_at)
                    VALUES ($1, $2, $3, $4, $5, now())
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
}
