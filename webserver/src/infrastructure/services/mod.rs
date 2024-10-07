use uuid::Uuid;

use crate::prelude::Result;

pub mod album;
pub mod archive;
pub mod cdn;
pub mod content_dir;
pub mod files;
pub mod auth_service;

pub fn parse_uuid(uuid: &str) -> Result<Uuid> {
    match Uuid::parse_str(&uuid) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(crate::error::Error::InvalidUuid(e)),
    }
}
