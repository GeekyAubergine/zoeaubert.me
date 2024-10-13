use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    domain::models::media::image::{Image, ImageUuid},
    error::DatabaseError,
    infrastructure::app_state::DatabaseConnection,
    prelude::Result,
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

impl From<ImageRepoEntity> for Image {
    fn from(entity: ImageRepoEntity) -> Self {
        let image = Image::new(
            &entity.uuid,
            &entity.url,
            &entity.alt,
            entity.width as u32,
            entity.height as u32,
        );

        if let Some(title) = entity.title {
            image.with_title(&title);
        }

        if let Some(description) = entity.description {
            image.with_description(&description);
        }

        if let Some(date) = entity.date {
            image.with_date(date);
        }

        if let Some(parent_permalink) = entity.parent_permalink {
            image.with_parent_permalink(&parent_permalink);
        }

        image
    }
}

impl From<&Image> for ImageRepoEntity {
    fn from(model: &Image) -> Self {
        ImageRepoEntity {
            uuid: model.uuid().clone(),
            url: model.url().to_string(),
            alt: model.alt().to_string(),
            width: model.width() as i32,
            height: model.height() as i32,
            title: model.title_inner().map(|t| t.to_string()),
            description: model.description().map(|d| d.to_string()),
            date: model.date().map(|d| *d),
            parent_permalink: model.parent_permalink().map(|p| p.to_string()),
            updated_at: model.updated_at().clone(),
        }
    }
}

#[async_trait::async_trait]
pub trait ImagesRepo {
    async fn find_by_uuid(&self, uuid: &ImageUuid) -> Result<Option<Image>>;
    async fn find_by_uuids(&self, uuids: &[ImageUuid]) -> Result<HashMap<ImageUuid, Image>>;
    async fn commit(&self, image: &Image) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ImagesRepoDb {
    database_connection: DatabaseConnection,
}

impl ImagesRepoDb {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }
}

#[async_trait::async_trait]
impl ImagesRepo for ImagesRepoDb {
    async fn find_by_uuid(&self, uuid: &ImageUuid) -> Result<Option<Image>> {
        let image = sqlx::query_as!(
            ImageRepoEntity,
            "
            SELECT uuid, url, alt, width, height, title, description, date, parent_permalink, updated_at
            FROM images
            WHERE uuid = $1
            ",
            uuid.0
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(image.map(|image| image.into()))
    }

    async fn find_by_uuids(&self, uuids: &[ImageUuid]) -> Result<HashMap<ImageUuid, Image>> {
        let images = sqlx::query_as!(
            ImageRepoEntity,
            "
            SELECT uuid, url, alt, width, height, title, description, date, parent_permalink, updated_at
            FROM images
            WHERE uuid = ANY($1)
            ",
            &uuids.iter().map(|uuid| uuid.0).collect::<Vec<Uuid>>()
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(images
            .into_iter()
            .map(|image| (image.uuid.into(), image.into()))
            .collect())
    }

    async fn commit(&self, image: &Image) -> Result<()> {
        if let Some(_) = self.find_by_uuid(&image.uuid).await? {
            let image: ImageRepoEntity = image.into();
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

        let image: ImageRepoEntity = image.into();

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
