use std::{collections::HashMap, hash::Hash, sync::Arc};

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    domain::models::{
        media::image::ImageUuid,
        micro_post::{MicroPost, MicroPostUuid},
    },
    error::DatabaseError,
    infrastructure::app_state::DatabaseConnection,
    prelude::Result,
};

#[async_trait::async_trait]
pub trait MicroPostsRepo {
    async fn find_by_uuid(&self, uuid: &MicroPostUuid) -> Result<Option<MicroPost>>;

    async fn find_all(&self) -> Result<HashMap<MicroPostUuid, MicroPost>>;

    async fn find_all_by_date(&self) -> Result<Vec<MicroPost>>;

    async fn find_by_slug(&self, slug: &str) -> Result<Option<MicroPost>>;

    async fn commit(&self, micro_post: &MicroPost) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MicroPostsRepoDb {
    database_connection: DatabaseConnection,
}

impl MicroPostsRepoDb {
    pub fn new(database_connection: DatabaseConnection) -> Self {
        Self {
            database_connection,
        }
    }
}

#[async_trait::async_trait]
impl MicroPostsRepo for MicroPostsRepoDb {
    async fn find_by_uuid(&self, uuid: &MicroPostUuid) -> Result<Option<MicroPost>> {
        let row = sqlx::query!(
            "
            SELECT * from micro_posts
            WHERE uuid = $1
            ",
            uuid.0,
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        match row {
            Some(micro_post) => Ok(Some(MicroPost::new(
                MicroPostUuid::new(micro_post.uuid),
                micro_post.slug,
                micro_post.date,
                micro_post.content,
                micro_post
                    .image_order
                    .into_iter()
                    .map(|i| i.into())
                    .collect(),
            ))),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<HashMap<MicroPostUuid, MicroPost>> {
        let micro_posts = sqlx::query!(
            "
            SELECT * from micro_posts
            ",
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        let mut micro_posts = micro_posts
            .into_iter()
            .map(|micro_post| {
                MicroPost::new(
                    MicroPostUuid::new(micro_post.uuid),
                    micro_post.slug,
                    micro_post.date,
                    micro_post.content,
                    micro_post
                        .image_order
                        .into_iter()
                        .map(|i| i.into())
                        .collect(),
                )
            })
            .collect::<Vec<MicroPost>>();

        micro_posts.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(micro_posts
            .into_iter()
            .map(|micro_post| (micro_post.uuid, micro_post))
            .collect::<HashMap<MicroPostUuid, MicroPost>>())
    }

    async fn find_all_by_date(&self) -> Result<Vec<MicroPost>> {
        let micro_posts = sqlx::query!(
            "
            SELECT * from micro_posts
            ORDER BY date
            ",
        )
        .fetch_all(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(micro_posts
            .into_iter()
            .map(|micro_post| {
                MicroPost::new(
                    MicroPostUuid::new(micro_post.uuid),
                    micro_post.slug,
                    micro_post.date,
                    micro_post.content,
                    micro_post
                        .image_order
                        .into_iter()
                        .map(|i| i.into())
                        .collect(),
                )
            })
            .collect())
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<MicroPost>> {
        let micro_post = sqlx::query!(
            "
            SELECT * from micro_posts
            WHERE slug = $1
            ",
            slug,
        )
        .fetch_optional(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(micro_post.map(|micro_post| {
            MicroPost::new(
                MicroPostUuid::new(micro_post.uuid),
                micro_post.slug,
                micro_post.date,
                micro_post.content,
                micro_post
                    .image_order
                    .into_iter()
                    .map(|i| i.into())
                    .collect(),
            )
        }))
    }

    async fn commit(&self, micro_post: &MicroPost) -> Result<()> {
        if let Some(_) = self.find_by_uuid(&micro_post.uuid).await? {
            sqlx::query!(
                "
                UPDATE micro_posts
                SET
                    slug = $1,
                    date = $2,
                    content = $3,
                    image_order = $4,
                    updated_at = NOW()
                ",
                micro_post.slug,
                micro_post.date,
                micro_post.content,
                &micro_post
                    .image_order
                    .iter()
                    .map(|i| i.into())
                    .collect::<Vec<Uuid>>()
            )
            .execute(&self.database_connection)
            .await
            .map_err(DatabaseError::from_query_error)?;

            return Ok(());
        }

        sqlx::query!(
            "
            INSERT INTO micro_posts
            (uuid, slug, date, content, image_order, updated_at)
            VALUES
            ($1, $2, $3, $4, $5, NOW())
            ",
            micro_post.uuid.0,
            micro_post.slug,
            micro_post.date,
            micro_post.content,
            &micro_post
                .image_order
                .iter()
                .map(|i| i.into())
                .collect::<Vec<Uuid>>()
        )
        .execute(&self.database_connection)
        .await
        .map_err(DatabaseError::from_query_error)?;

        Ok(())
    }
}
