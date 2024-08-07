use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    domain::models::silly_name::SillyName,
    infrastructure::{app_state::AppState, repos::silly_names_repo::SillyNameDbEntity},
    prelude::Result,
};

impl From<&SillyName> for SillyNameDbEntity {
    fn from(silly_name: &SillyName) -> Self {
        Self {
            uuid: silly_name.uuid,
            name: silly_name.name.to_string(),
            deleted_at: silly_name.deleted_at,
        }
    }
}

impl From<&SillyNameDbEntity> for SillyName {
    fn from(silly_name: &SillyNameDbEntity) -> Self {
        Self {
            uuid: silly_name.uuid,
            name: silly_name.name.to_string(),
            deleted_at: silly_name.deleted_at,
        }
    }
}

pub struct SillyNamesQueryService;

impl SillyNamesQueryService {
    pub async fn find_all(state: &AppState) -> Result<HashMap<Uuid, SillyName>> {
        let entities = state.silly_names_repo().find_all().await?;

        Ok(entities
            .iter()
            .map(|(k, v)| (k.clone(), v.into()))
            .collect())
    }

    pub async fn delete(state: &AppState, entity_uuid: Uuid) -> Result<()> {
        state.silly_names_repo().delete(entity_uuid).await
    }

    pub async fn commit(state: &AppState, entity: &SillyName) -> Result<()> {
        state.silly_names_repo().commit(&entity.into()).await
    }
}
