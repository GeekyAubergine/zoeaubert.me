use crate::domain::models::referral::{Referral, Referrals};
use crate::prelude::*;
use crate::services::file_service::{FileService, ReadableFile};
use crate::{domain::models::now_text::NowText, services::ServiceContext};

const FILE_NAME: &str = "referrals.json";

pub fn process_referrals(ctx: &ServiceContext) -> Result<Referrals> {
    let referrals: Vec<Referral> = FileService::content(FILE_NAME.into()).read_json()?;

    Ok(Referrals { referrals })
}
