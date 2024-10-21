use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::domain::queries::silly_names_queries::commit_silly_names;
use crate::domain::state::State;
use crate::infrastructure::utils::file_system::{make_content_file_path, read_csv_file};
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SillyNamesFileRecord {
    pub name: String,
    pub creator: Option<String>,
}

const FILE_NAME: &str = "silly_names.csv";

pub async fn update_silly_names_command(state: &impl State) -> Result<()> {
    debug!("Update silly names");
    let silly_names: Vec<SillyNamesFileRecord> =
        read_csv_file(&make_content_file_path(&Path::new(FILE_NAME))).await?;

    let silly_names: Vec<String> = silly_names.into_iter().map(|record| record.name).collect();

    commit_silly_names(state, silly_names).await?;

    Ok(())
}
