use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::prelude::{self, Result};

use crate::{error::DatabaseError, infrastructure::app_state::DatabaseConnection};

pub struct AlbumPhotoRepoEntity {
    uuid: Uuid,
    album_uuid: Uuid,
    small_image_uuid: Uuid,
    large_image_uuid: Uuid,
    full_image_uuid: Uuid,
    file_name: String,
    featured: bool,
    updated_at: DateTime<Utc>,
}

impl AlbumPhotoRepoEntity {
    pub fn new(
        uuid: Uuid,
        album_uuid: Uuid,
        small_image_uuid: Uuid,
        large_image_uuid: Uuid,
        full_image_uuid: Uuid,
        file_name: String,
        featured: bool,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            uuid,
            album_uuid,
            small_image_uuid,
            large_image_uuid,
            full_image_uuid,
            file_name,
            featured,
            updated_at,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn album_uuid(&self) -> &Uuid {
        &self.album_uuid
    }

    pub fn small_image_uuid(&self) -> &Uuid {
        &self.small_image_uuid
    }

    pub fn large_image_uuid(&self) -> &Uuid {
        &self.large_image_uuid
    }

    pub fn full_image_uuid(&self) -> &Uuid {
        &self.full_image_uuid
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn featured(&self) -> bool {
        self.featured
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone)]
pub struct AlbumPhotosRepo {
    database_connection: DatabaseConnection,
}

impl AlbumPhotosRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_by_uuid(&self, uuid: &Uuid) -> Result<Option<AlbumPhotoRepoEntity>> {
        sqlx::query_as!(
            AlbumPhotoRepoEntity,
            "
            SELECT
                uuid,
                album_uuid,
                small_image_uuid,
                large_image_uuid,
                full_image_uuid,
                file_name,
                featured,
                updated_at
            FROM
                album_photos
            WHERE
                uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)
    }

    pub async fn find_by_album_uuid(
        &self,
        album_uuid: &Uuid,
    ) -> Result<HashMap<Uuid, AlbumPhotoRepoEntity>> {
        let album_photos = sqlx::query_as!(
            AlbumPhotoRepoEntity,
            "
            SELECT
                uuid,
                album_uuid,
                small_image_uuid,
                large_image_uuid,
                full_image_uuid,
                file_name,
                featured,
                updated_at
            FROM
                album_photos
            WHERE
                album_uuid = $1
            ",
            album_uuid
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(album_photos
            .into_iter()
            .map(|album_photo| (album_photo.uuid, album_photo))
            .collect())
    }

    pub async fn commit(&self, entity: &AlbumPhotoRepoEntity) -> Result<()> {
        sqlx::query!(
            "
            INSERT INTO album_photos (
                uuid,
                album_uuid,
                small_image_uuid,
                large_image_uuid,
                full_image_uuid,
                file_name,
                featured,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ",
            entity.uuid,
            entity.album_uuid,
            entity.small_image_uuid,
            entity.large_image_uuid,
            entity.full_image_uuid,
            entity.file_name,
            entity.featured
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
