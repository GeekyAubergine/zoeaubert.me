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
    error::{DatabaseError, LegoMinifiguresError},
    get_json,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
    ONE_HOUR_CACHE_PERIOD,
};

const NO_REFETCH_DURATION: Duration = ONE_HOUR_CACHE_PERIOD;

struct SelectedRow {
    id: String,
    name: String,
    category: String,
    owned_in_sets: i64,
    owned_loose: i64,
    total_owned: i64,
    image_url: String,
    updated_at: DateTime<Utc>,
}

impl From<SelectedRow> for LegoMinifig {
    fn from(row: SelectedRow) -> LegoMinifig {
        LegoMinifig::new(
            row.id,
            row.name,
            row.category,
            row.owned_in_sets as u32,
            row.owned_loose as u32,
            row.total_owned as u32,
            row.image_url,
            row.updated_at,
        )
    }
}

#[derive(Debug, Clone)]
pub struct LegoMinifigsRepo {
    database_connection: DatabaseConnection,
}

impl LegoMinifigsRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self { database_connection }
    }

    pub async fn find_all(&self) -> Result<Vec<LegoMinifig>> {
        let minifigs = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, category, owned_in_sets, owned_loose, total_owned, image_url, updated_at
            FROM lego_minifigs
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let minifigs = minifigs.into_iter().map(LegoMinifig::from).collect();

        Ok(minifigs)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<LegoMinifig>> {
        let minifig = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, category, owned_in_sets, owned_loose, total_owned, image_url, updated_at
            FROM lego_minifigs
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        match minifig {
            Some(row) => Ok(Some(row.into())),
            None => Ok(None),
        }
    }

    pub async fn find_by_most_owned(&self) -> Result<Vec<LegoMinifig>> {
        let minifigs = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, category, owned_in_sets, owned_loose, total_owned, image_url, updated_at
            FROM lego_minifigs
            ORDER BY total_owned DESC
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let minifigs = minifigs.into_iter().map(LegoMinifig::from).collect();

        Ok(minifigs)
    }

    pub async fn find_all_sorted_by_category_and_name(&self) -> Result<Vec<LegoMinifig>> {
        let minifigs = sqlx::query_as!(
            SelectedRow,
            "
            SELECT id, name, category, owned_in_sets, owned_loose, total_owned, image_url, updated_at
            FROM lego_minifigs
            ORDER BY category, name
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let minifigs = minifigs.into_iter().map(LegoMinifig::from).collect();

        Ok(minifigs)
    }

    pub async fn find_total_owned(&self) -> Result<u32> {
        let row = sqlx::query!(
            "
            SELECT SUM(total_owned) as total_owned
            FROM lego_minifigs
            "
        )
        .fetch_one(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let total_owned = row.total_owned.ok_or(LegoMinifiguresError::unable_to_calculate_total_minifigures_count())?;

        Ok(total_owned as u32)
    }

    pub async fn commit(&self, minfig: &LegoMinifig) -> Result<()> {
        if let Some(_) = self.find_by_id(minfig.id()).await? {
            sqlx::query!(
                "
                UPDATE lego_minifigs
                SET name = $2, category = $3, owned_in_sets = $4, owned_loose = $5, total_owned = $6, image_url = $7, updated_at = NOW()
                WHERE id = $1
                ",
                minfig.id(),
                minfig.name(),
                minfig.category(),
                minfig.owned_in_sets() as i64,
                minfig.owned_loose() as i64,
                minfig.total_owned() as i64,
                minfig.image_url(),
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        sqlx::query!(
            "
            INSERT INTO lego_minifigs (id, name, category, owned_in_sets, owned_loose, total_owned, image_url, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ",
            minfig.id(),
            minfig.name(),
            minfig.category(),
            minfig.owned_in_sets() as i64,
            minfig.owned_loose() as i64,
            minfig.total_owned() as i64,
            minfig.image_url(),
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
