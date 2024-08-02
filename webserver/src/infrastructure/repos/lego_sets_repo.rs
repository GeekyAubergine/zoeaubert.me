use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::models::lego::{LegoMinifig, LegoSet},
    error::{DatabaseError, LegoSetsError},
    get_json,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
    ONE_HOUR_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

struct SelectedRow {
    id: i64,
    name: String,
    number: String,
    category: String,
    pieces: i64,
    image_url: String,
    thumbnail_url: String,
    link: String,
    quantity: i64,
    updated_at: DateTime<Utc>,
}

impl From<SelectedRow> for LegoSet {
    fn from(row: SelectedRow) -> Self {
        LegoSet::new(
            row.id as u32,
            row.name,
            row.number,
            row.category,
            row.pieces as u32,
            row.image_url,
            row.thumbnail_url,
            row.link,
            row.quantity as u32,
            row.updated_at,
        )
    }
}

#[derive(Debug, Clone)]
pub struct LegoSetsRepo {
    database_connection: DatabaseConnection,
}

impl LegoSetsRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<LegoSet>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, number, category, pieces, image_url, thumbnail_url, link, quantity, updated_at
            FROM lego_sets"
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().map(LegoSet::from).collect())
    }

    pub async fn find_by_id(&self, id: u32) -> Result<Option<LegoSet>> {
        let row = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, number, category, pieces, image_url, thumbnail_url, link, quantity, updated_at
            FROM lego_sets
            WHERE id = $1",
            id as i64
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row.map(LegoSet::from))
    }

    pub async fn find_all_sort_by_most_pieces(&self) -> Result<Vec<LegoSet>> {
        let rows = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, number, category, pieces, image_url, thumbnail_url, link, quantity, updated_at
            FROM lego_sets
            ORDER BY pieces DESC"
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().map(LegoSet::from).collect())
    }

    pub async fn find_total_pieces(&self) -> Result<u32> {
        let row = sqlx::query!("SELECT SUM(pieces) as total_pieces FROM lego_sets")
            .fetch_one(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

        let pieces = row
            .total_pieces
            .ok_or(LegoSetsError::unable_to_calculate_total_piece_count())?;

        Ok(pieces as u32)
    }

    pub async fn find_total_owned(&self) -> Result<u32> {
        let row = sqlx::query!("SELECT SUM(quantity) as total_owned FROM lego_sets")
            .fetch_one(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

        let owned = row
            .total_owned
            .ok_or(LegoSetsError::unable_to_calculate_total_owned_count())?;

        Ok(owned as u32)
    }

    pub async fn find_most_recently_updated_at(&self) -> Result<Option<DateTime<Utc>>> {
        let row = sqlx::query!(
            "
            SELECT updated_at
            FROM lego_sets
            ORDER BY updated_at DESC
            LIMIT 1
            "
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(row.map(|row| row.updated_at))
    }

    pub async fn commit(&self, lego_set: &LegoSet) -> Result<()> {
        if let Some(_) = self.find_by_id(lego_set.id()).await? {
            sqlx::query!(
                "
                UPDATE lego_sets
                SET name = $2, number = $3, category = $4, pieces = $5, image_url = $6, thumbnail_url = $7, link = $8, quantity = $9, updated_at = NOW()
                WHERE id = $1
                ",
                lego_set.id() as i64,
                lego_set.name(),
                lego_set.number(),
                lego_set.category(),
                lego_set.pieces() as i64,
                lego_set.image(),
                lego_set.thumbnail(),
                lego_set.link(),
                lego_set.quantity() as i64,
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        sqlx::query!("
            INSERT INTO lego_sets (id, name, number, category, pieces, image_url, thumbnail_url, link, quantity, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
            ",
            lego_set.id() as i64,
            lego_set.name(),
            lego_set.number(),
            lego_set.category(),
            lego_set.pieces() as i64,
            lego_set.image(),
            lego_set.thumbnail(),
            lego_set.link(),
            lego_set.quantity() as i64,
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
