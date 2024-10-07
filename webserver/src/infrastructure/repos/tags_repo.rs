use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    domain::models::tag::Tag,
    error::DatabaseError,
    infrastructure::app_state::DatabaseConnection,
    prelude::{self, Result},
};

pub struct TagRepoEntity {
    entity_uuid: Uuid,
    tag: String,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl TagRepoEntity {
    pub fn entity_uuid(&self) -> &Uuid {
        &self.entity_uuid
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
    }
}

pub struct TagEntity {
    pub entity_uuid: Uuid,
    pub tag: Tag,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<TagRepoEntity> for TagEntity {
    fn from(entity: TagRepoEntity) -> Self {
        Self {
            entity_uuid: entity.entity_uuid,
            tag: Tag::from_string(&entity.tag),
            updated_at: entity.updated_at,
            deleted_at: entity.deleted_at,
        }
    }
}

#[async_trait::async_trait]
pub trait TagsRepo {
    async fn find_by_entity_uuid(&self, entity_uuid: &Uuid) -> Result<Vec<TagEntity>>;

    async fn find_by_entity_uuid_including_deleted(
        &self,
        entity_uuid: &Uuid,
    ) -> Result<Vec<TagEntity>>;

    async fn find_by_entity_uuids(
        &self,
        entity_uuids: &[Uuid],
    ) -> Result<HashMap<Uuid, Vec<TagEntity>>>;

    async fn find_by_tag(&self, tag: &str) -> Result<Vec<TagEntity>>;

    async fn find_tag_counts(&self) -> Result<Vec<(String, i64)>>;

    async fn delete(&self, entity_uuid: &Uuid, tag: &str) -> Result<()>;

    async fn commit(&self, entity_uuid: &Uuid, tag: &str) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct TagsRepoDb {
    database_connection: DatabaseConnection,
}

impl TagsRepoDb {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }
}

#[async_trait::async_trait]
impl TagsRepo for TagsRepoDb {
    async fn find_by_entity_uuid(&self, entity_uuid: &Uuid) -> Result<Vec<TagEntity>> {
        let rows = sqlx::query_as!(
            TagRepoEntity,
            "
            SELECT entity_uuid, tag, updated_at, deleted_at
            FROM tags
            WHERE entity_uuid = $1 AND deleted_at IS NULL
            ",
            entity_uuid
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let rows = rows
            .into_iter()
            .map(|row| TagEntity::from(row))
            .collect();

        Ok(rows)
    }

    async fn find_by_entity_uuid_including_deleted(
        &self,
        entity_uuid: &Uuid,
    ) -> Result<Vec<TagEntity>> {
        let rows = sqlx::query_as!(
            TagRepoEntity,
            "
            SELECT entity_uuid, tag, updated_at, deleted_at
            FROM tags
            WHERE entity_uuid = $1
            ",
            entity_uuid
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let rows = rows
            .into_iter()
            .map(|row| TagEntity::from(row))
            .collect();

        Ok(rows)
    }

    async fn find_by_entity_uuids(
        &self,
        entity_uuids: &[Uuid],
    ) -> Result<HashMap<Uuid, Vec<TagEntity>>> {
        let rows = sqlx::query_as!(
            TagRepoEntity,
            "
            SELECT entity_uuid, tag, updated_at, deleted_at
            FROM tags
            WHERE entity_uuid = ANY($1) AND deleted_at IS NULL
            ",
            entity_uuids
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(rows.into_iter().fold(HashMap::new(), |mut map, row| {
            map.entry(row.entity_uuid)
                .or_insert_with(Vec::new)
                .push(
                    row.into()
                );
            map
        }))
    }

    async fn find_by_tag(&self, tag: &str) -> Result<Vec<TagEntity>> {
        let rows = sqlx::query_as!(
            TagRepoEntity,
            "
            SELECT entity_uuid, tag, updated_at, deleted_at
            FROM tags
            WHERE tag = $1
            ",
            tag
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let rows = rows.into_iter().map(|row| TagEntity::from(row)).collect();

        Ok(rows)
    }

    async fn find_tag_counts(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query!(
            "
            SELECT tag, count(*)
            FROM tags
            WHERE deleted_at IS NULL
            GROUP BY tag
            "
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let tag_counts = rows
            .into_iter()
            .filter_map(|row| Some((row.tag, row.count?)))
            .collect::<Vec<(String, i64)>>();

        Ok(tag_counts)
    }

    async fn delete(&self, entity_uuid: &Uuid, tag: &str) -> Result<()> {
        sqlx::query!(
            "
            UPDATE tags
            SET deleted_at = now()
            WHERE entity_uuid = $1 AND tag = $2
            ",
            entity_uuid,
            tag
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }

    async fn commit(&self, entity_uuid: &Uuid, tag: &str) -> Result<()> {
        sqlx::query!(
            "
            INSERT INTO tags (entity_uuid, tag, updated_at)
            VALUES ($1, $2, now())
            ",
            entity_uuid,
            tag
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
