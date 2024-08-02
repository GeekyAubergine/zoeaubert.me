use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    domain::models::album::AlbumPhoto, error::AlbumError, infrastructure::app_state::AppState,
    prelude::Result,
};

use super::{images_query_service::ImagesQueryService, tags_query_service::TagsQueryService};

pub struct AlbumPhotosQueryService;

impl AlbumPhotosQueryService {
    pub async fn find_by_uuid(
        state: &AppState,
        album_photo_uuid: &Uuid,
    ) -> Result<Option<AlbumPhoto>> {
        let album_data = state
            .album_photos_repo()
            .find_by_uuid(album_photo_uuid)
            .await?;

        let album_data = match album_data {
            Some(album_data) => album_data,
            None => return Ok(None),
        };

        let image_uuids = vec![
            album_data.small_image_uuid().clone(),
            album_data.large_image_uuid().clone(),
            album_data.full_image_uuid().clone(),
        ];

        let images = ImagesQueryService::find_by_uuids(state, &image_uuids).await?;

        let full_image = images.get(album_data.full_image_uuid()).ok_or(
            AlbumError::album_photo_image_not_found(album_photo_uuid.clone(), "full".to_string()),
        )?;
        let large_image = images.get(album_data.large_image_uuid()).ok_or(
            AlbumError::album_photo_image_not_found(album_photo_uuid.clone(), "large".to_string()),
        )?;
        let small_image = images.get(album_data.small_image_uuid()).ok_or(
            AlbumError::album_photo_image_not_found(album_photo_uuid.clone(), "small".to_string()),
        )?;

        let tags = TagsQueryService::find_tags_for_entity(state, album_photo_uuid, false).await?;

        Ok(Some(AlbumPhoto::new(
            *album_data.uuid(),
            *album_data.album_uuid(),
            small_image.clone(),
            large_image.clone(),
            full_image.clone(),
            album_data.file_name().to_string(),
            tags,
            album_data.featured(),
        )))
    }

    pub async fn find_by_album_uuid(
        state: &AppState,
        album_uuid: &Uuid,
    ) -> Result<HashMap<Uuid, AlbumPhoto>> {
        let album_data = state
            .album_photos_repo()
            .find_by_album_uuid(album_uuid)
            .await?;

        let image_uuids: Vec<Uuid> = album_data.values().fold(vec![], |mut acc, album_photo| {
            acc.push(album_photo.full_image_uuid().clone());
            acc.push(album_photo.large_image_uuid().clone());
            acc.push(album_photo.small_image_uuid().clone());
            acc
        });

        let images = ImagesQueryService::find_by_uuids(state, &image_uuids).await?;

        let album_photo_uuids = album_data.keys().cloned().collect::<Vec<Uuid>>();

        let tags = TagsQueryService::find_tags_for_entities(state, &album_photo_uuids).await?;

        let mut album_photos = HashMap::new();

        for album_photo in album_data.values() {
            let full_image = images.get(album_photo.full_image_uuid()).ok_or(
                AlbumError::album_photo_image_not_found(
                    album_photo.uuid().clone(),
                    "full".to_string(),
                ),
            )?;
            let large_image = images.get(album_photo.large_image_uuid()).ok_or(
                AlbumError::album_photo_image_not_found(
                    album_photo.uuid().clone(),
                    "large".to_string(),
                ),
            )?;
            let small_image = images.get(album_photo.small_image_uuid()).ok_or(
                AlbumError::album_photo_image_not_found(
                    album_photo.uuid().clone(),
                    "small".to_string(),
                ),
            )?;

            let tags = match tags.get(album_photo.uuid()) {
                Some(tags) => tags.clone(),
                None => vec![],
            };

            album_photos.insert(
                *album_photo.uuid(),
                AlbumPhoto::new(
                    *album_photo.uuid(),
                    *album_photo.album_uuid(),
                    small_image.clone(),
                    large_image.clone(),
                    full_image.clone(),
                    album_photo.file_name().to_string(),
                    tags,
                    album_photo.featured(),
                ),
            );
        }

        Ok(album_photos)
    }
}
