use std::collections::HashMap;

use axum::http::StatusCode;
use axum::{extract::State, Json};
use tracing::{debug, info};

use crate::prelude::Result;
use crate::ResponseResult;
use crate::{
    domain::models::silly_name::SillyName,
    infrastructure::{
        app_state::AppState, query_services::silly_names_query_service::SillyNamesQueryService,
    },
};

pub async fn update_silly_names_command(
    state: &AppState,
    new_silly_names: &[String],
) -> Result<()> {
    info!("Updating silly names");
    let existing_silly_names = SillyNamesQueryService::find_all(&state).await?;
    let existing_by_name = existing_silly_names
        .values()
        .map(|n| (n.name.clone(), n.clone()))
        .collect::<HashMap<String, SillyName>>();

    let silly_names_to_delete = existing_silly_names
        .values()
        .filter(|n| !new_silly_names.contains(&n.name))
        .collect::<Vec<&SillyName>>();

    for name in silly_names_to_delete {
        debug!("Deleting silly name: {:?}", name);
        SillyNamesQueryService::delete(&state, name.uuid).await?;
    }

    for name in new_silly_names.iter() {
        match existing_by_name.get(name) {
            Some(existing) => {
                let mut existing = existing.clone();
                debug!("Silly name already exists: {:?}", existing);
                existing.deleted_at = None;
                SillyNamesQueryService::commit(&state, &existing).await?;
            }
            None => {
                debug!("Inserting silly name: {:?}", name);
                SillyNamesQueryService::commit(&state, &SillyName::from_name(name)).await?;
            }
        }
    }

    Ok(())
}
