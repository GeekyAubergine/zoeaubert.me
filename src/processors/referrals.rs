use crate::domain::models::referral::{Referral, Referrals};
use crate::prelude::*;
use crate::services::file_service::{FileService, ReadableFile};

const FILE_NAME: &str = "referrals.json";

pub fn load_referrals() -> Result<Referrals> {
    let referrals: Vec<Referral> = FileService::content(FILE_NAME.into()).read_json()?;

    Ok(Referrals { referrals })
}
