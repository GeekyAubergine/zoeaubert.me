use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    domain::models::media::image::Image, error::DatabaseError,
    infrastructure::app_state::DatabaseConnection, prelude::Result,
};

pub struct ImageRepoEntity {
    pub uuid: Uuid,
    pub url: String,
    pub alt: String,
    pub width: i32,
    pub height: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub parent_permalink: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ImagesRepo {
    database_connection: DatabaseConnection,
}

impl ImagesRepo {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }

    pub async fn find_by_uuid(&self, uuid: &Uuid) -> Result<Option<ImageRepoEntity>> {
        sqlx::query_as!(
            ImageRepoEntity,
            "
            SELECT uuid, url, alt, width, height, title, description, date, parent_permalink, updated_at
            FROM images
            WHERE uuid = $1
            ",
            uuid
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)
    }

    pub async fn find_by_uuids(&self, uuids: &[Uuid]) -> Result<HashMap<Uuid, ImageRepoEntity>> {
        let images = sqlx::query_as!(
            ImageRepoEntity,
            "
            SELECT uuid, url, alt, width, height, title, description, date, parent_permalink, updated_at
            FROM images
            WHERE uuid = ANY($1)
            ",
            uuids
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(images
            .into_iter()
            .map(|image| (image.uuid, image))
            .collect())
    }

    pub async fn commit(&self, image: &ImageRepoEntity) -> Result<()> {
        if let Some(_) = self.find_by_uuid(&image.uuid).await? {
            sqlx::query!("
                UPDATE images
                SET url = $2, alt = $3, width = $4, height = $5, title = $6, description = $7, date = $8, parent_permalink = $9, updated_at = now()
                WHERE uuid = $1
                ",
                image.uuid,
                image.url,
                image.alt,
                image.width,
                image.height,
                image.title,
                image.description,
                image.date,
                image.parent_permalink
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        sqlx::query!("
            INSERT INTO images (uuid, url, alt, width, height, title, description, date, parent_permalink, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, now())
            ",
            image.uuid,
            image.url,
            image.alt,
            image.width,
            image.height,
            image.title,
            image.description,
            image.date,
            image.parent_permalink
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
