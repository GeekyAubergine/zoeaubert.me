use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    infrastructure::{app_state::DatabaseConnection, config::Config},
    prelude::*,
};

const FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Clone)]
pub struct SillyNameDbEntity {
    pub uuid: Uuid,
    pub name: String,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SillyNamesRepo {
    database_connection: DatabaseConnection,
}

impl SillyNamesRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<SillyNameDbEntity>> {
        sqlx::query_as!(
            SillyNameDbEntity,
            "
            SELECT * FROM silly_names
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<SillyNameDbEntity>> {
        sqlx::query_as!(
            SillyNameDbEntity,
            "
            SELECT * FROM silly_names
            WHERE name = $1
            ",
            name
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)
    }

    pub async fn find_all(&self) -> Result<HashMap<Uuid, SillyNameDbEntity>> {
        let names = sqlx::query_as!(
            SillyNameDbEntity,
            "
            SELECT * FROM silly_names
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(names
            .into_iter()
            .map(|silly_name| (silly_name.uuid, silly_name))
            .collect())
    }

    pub async fn delete(&self, uuid: Uuid) -> Result<()> {
        sqlx::query!(
            "
            UPDATE silly_names
            SET deleted_at = NOW()
            WHERE uuid = $1
            ",
            uuid
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }

    pub async fn commit(&self, silly_name: &SillyNameDbEntity) -> Result<()> {
        if let Some(_) = self.find_by_uuid(silly_name.uuid).await? {
            sqlx::query!(
                "
                UPDATE silly_names
                SET name = $1, deleted_at = $2
                WHERE uuid = $3
                ",
                silly_name.name,
                silly_name.deleted_at,
                silly_name.uuid
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        println!("committing silly name: {:?}", silly_name);
        sqlx::query!(
            "
            INSERT INTO silly_names (uuid, name, deleted_at)
            VALUES ($1, $2, $3)
            ",
            silly_name.uuid,
            silly_name.name,
            silly_name.deleted_at
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
