use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    domain::models::{
        media::{image::{Image, ImageUuid}, Media, MediaUuid},
        UuidIdentifiable,
    },
    infrastructure::{
        app_state::AppState,
        query_services::images_query_service::find_images_by_uuids,
        repos::images_repo::{ImageRepoEntity, ImagesRepo, ImagesRepoDb},
    },
    prelude::Result,
};

use super::images_query_service::commit_image;

pub async fn find_media_by_uuids(
    uuids: &[MediaUuid],
    repo: &impl ImagesRepo,
) -> Result<HashMap<MediaUuid, Media>> {
    let images = find_images_by_uuids(
        &uuids.iter().map(|uuid| uuid.into()).collect::<Vec<ImageUuid>>(),
        repo,
    )
    .await?;

    let mut media = HashMap::new();

    for (uuid, image) in images {
        media.insert(uuid.into(), image.into());
    }

    Ok(media)
}

pub async fn commit_media(media: &Media, repo: &impl ImagesRepo) -> Result<()> {
    match media {
        Media::Image(image) => commit_image(image, repo).await,
    }
}

// pub struct MediaQueryService;

// impl MediaQueryService {
//     pub async fn find_by_uuid(state: &AppState, uuid: &Uuid) -> Result<Option<Media>> {
//         let image = ImagesQueryService::find_by_uuid(state, uuid).await?;

//         match image {
//             Some(image) => Ok(Some(image.into())),
//             None => Ok(None),
//         }
//     }

//     pub async fn find_by_uuids(state: &AppState, uuids: &[Uuid]) -> Result<HashMap<Uuid, Media>> {
//         let images = ImagesQueryService::find_by_uuids(state, uuids).await?;

//         let mut map = HashMap::new();

//         for (uuid, image) in images {
//             map.insert(uuid, image.into());
//         }

//         Ok(map)
//     }

//     pub async fn update(state: &AppState, media: &Media) -> Result<()> {
//         match media {
//             Media::Image(image) => ImagesQueryService::update(state, image).await,
//         }
//     }
// }
