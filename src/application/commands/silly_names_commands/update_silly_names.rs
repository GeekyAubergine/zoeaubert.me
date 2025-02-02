use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::domain::repositories::SillyNamesRepo;
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SillyNamesFileRecord {
    pub name: String,
    pub creator: Option<String>,
}

const FILE_NAME: &str = "silly_names.csv";

pub async fn update_silly_names_command(state: &impl State) -> Result<()> {
    debug!("Update silly names");
    let silly_names: Vec<SillyNamesFileRecord> = state
        .file_service()
        .read_csv_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(FILE_NAME)),
        )
        .await?;

    let silly_names: Vec<String> = silly_names.into_iter().map(|record| record.name).collect();

    state.silly_names_repo().commit(silly_names).await?;

    Ok(())
}
