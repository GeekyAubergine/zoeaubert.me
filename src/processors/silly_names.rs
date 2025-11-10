use std::path::PathBuf;

use serde::Deserialize;

use crate::domain::models::referral::{Referral, Referrals};
use crate::domain::models::silly_names::SillyNames;
use crate::prelude::*;
use crate::services::file_service::{FileService, ReadableFile};
use crate::{domain::models::now_text::NowText, services::ServiceContext};

const FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Deserialize)]
pub struct SillyNamesFileRecord {
    pub name: String,
    pub creator: Option<String>,
}

pub fn load_silly_names(ctx: &ServiceContext) -> Result<SillyNames> {
    let silly_names: Vec<SillyNamesFileRecord> =
        FileService::content(FILE_NAME.into()).read_csv()?;

    let silly_names: Vec<String> = silly_names.into_iter().map(|record| record.name).collect();

    Ok(SillyNames { silly_names })
}
