use serde::Deserialize;

use crate::domain::models::referral::{Referral, Referrals};
use crate::domain::models::silly_names::SillyNames;
use crate::prelude::*;
use crate::services::file_service::FilePath;
use crate::{
    domain::models::now_text::NowText,
    services::{file_service::File, ServiceContext},
};

const FILE_NAME: &str = "silly_names.csv";

#[derive(Debug, Deserialize)]
pub struct SillyNamesFileRecord {
    pub name: String,
    pub creator: Option<String>,
}

pub async fn process_silly_names(ctx: &ServiceContext) -> Result<SillyNames> {
    let silly_names: Vec<SillyNamesFileRecord> = File::from_path(FilePath::content(FILE_NAME))
        .read_as_csv()
        .await?;

    let silly_names: Vec<String> = silly_names.into_iter().map(|record| record.name).collect();

    Ok(SillyNames { silly_names })
}
