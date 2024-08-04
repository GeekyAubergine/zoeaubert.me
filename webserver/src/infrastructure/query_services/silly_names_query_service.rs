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

fn calculate_updated_silly_names_from_strings(
    db_entities: &HashMap<String, SillyName>,
    updated_silly_names: Vec<String>,
) -> HashMap<String, SillyName> {
    let mut entities = HashMap::new();

    let (names_in_new_set, names_not_in_new_set): (Vec<&SillyName>, Vec<&SillyName>) = db_entities
        .values()
        .partition(|e| updated_silly_names.contains(&e.name));

    for name in names_not_in_new_set {
        let mut name = name.clone();
        name.deleted_at = Some(chrono::Utc::now());
        entities.insert(name.name.to_string(), name.clone());
    }

    for name in names_in_new_set {
        entities.insert(name.name.to_string(), name.clone());
    }

    for name in updated_silly_names {
        if !entities.contains_key(&name) {
            entities.insert(name.to_string(), SillyName::from_name(name.as_str()));
        }
    }

    entities
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
