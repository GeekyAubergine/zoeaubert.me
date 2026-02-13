use tracing::info;

use crate::{
    domain::models::credits::Credits,
    prelude::*,
    services::file_service::{FileService, ReadableFile},
};

pub const CREDITS_FILE: &str = "credits.yml";

pub fn load_credits() -> Result<Credits> {
    info!("Processing Credits");
    let mut credits: Credits = FileService::content(CREDITS_FILE.into()).read_yaml()?;

    credits.sort_by_key(|c| c.name.clone());

    Ok(credits)
}
