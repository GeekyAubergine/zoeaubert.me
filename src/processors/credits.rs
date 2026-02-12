use chrono::DateTime;
use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    domain::models::{
        credits::Credits,
        projects::{Project, Projects},
    },
    prelude::*,
    services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile},
        media_service::MediaService,
    },
};

pub const CREDITS_FILE: &str = "credits.yml";

pub fn load_credits(ctx: &ServiceContext) -> Result<Credits> {
    info!("Processing Credits");
    let mut credits: Credits = FileService::content(CREDITS_FILE.into()).read_yaml()?;

    credits.sort_by_key(|c| c.name.clone());

    Ok(credits)
}
