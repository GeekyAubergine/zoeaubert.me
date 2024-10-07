use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    domain::models::media::image::{Image, ImageUuid},
    infrastructure::{
        app_state::AppState,
        repos::images_repo::{ImageRepoEntity, ImagesRepo},
    },
    prelude::Result,
};

pub async fn find_image_by_uuid(
    uuid: &ImageUuid,
    images_repo: &impl ImagesRepo,
) -> Result<Option<Image>> {
    images_repo.find_by_uuid(uuid).await
}

pub async fn find_images_by_uuids(
    uuids: &[ImageUuid],
    images_repo: &impl ImagesRepo,
) -> Result<HashMap<ImageUuid, Image>> {
    images_repo.find_by_uuids(uuids).await
}

pub async fn commit_image(image: &Image, images_repo: &impl ImagesRepo) -> Result<()> {
    images_repo.commit(image).await
}
