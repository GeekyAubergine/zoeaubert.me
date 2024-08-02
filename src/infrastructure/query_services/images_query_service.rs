use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    domain::models::media::image::Image,
    infrastructure::{
        app_state::AppState,
        repos::images_repo::{ImageRepoEntity, ImagesRepo},
    },
    prelude::Result,
};

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

pub struct ImagesQueryService;

impl ImagesQueryService {
    pub async fn find_by_uuid(state: &AppState, uuid: &Uuid) -> Result<Option<Image>> {
        let entity = state.images_repo().find_by_uuid(uuid).await?;

        Ok(entity.map(|e| e.into()))
    }

    pub async fn find_by_uuids(state: &AppState, uuids: &[Uuid]) -> Result<HashMap<Uuid, Image>> {
        let entities = state.images_repo().find_by_uuids(uuids).await?;

        Ok(entities
            .into_iter()
            .map(|(uuid, entity)| (uuid, entity.into()))
            .collect())
    }

    pub async fn update(state: &AppState, image: &Image) -> Result<()> {
        state.images_repo().commit(&image.into()).await
    }
}
