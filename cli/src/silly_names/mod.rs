use serde::{Deserialize, Serialize};

use crate::{content_folder_path, error::SillyNamesError, prelude::Result};

pub mod upload_silly_names;

pub const SILLY_NAMES_FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Serialize, Deserialize)]
pub struct SillyNamesFileRecord {
    pub name: String,
    pub creator: Option<String>,
}

const FILE_NAME: &str = "silly_names.csv";

pub async fn read_silly_names_file() -> Result<Vec<SillyNamesFileRecord>> {
    let path = content_folder_path().join(FILE_NAME);

    let mut reader = csv::Reader::from_path(path).map_err(SillyNamesError::unable_to_read_csv)?;
    let mut records = Vec::new();
    for record in reader.deserialize() {
        let record: SillyNamesFileRecord = record.map_err(SillyNamesError::unable_to_parse_csv)?;
        records.push(record);
    }
    Ok(records)
}
