use crate::domain::models::referral::{Referral, Referrals};
use crate::prelude::*;
use crate::services::file_service::FilePath;
use crate::{
    domain::models::now_text::NowText,
    services::{file_service::File, ServiceContext},
};

const FILE_NAME: &str = "referrals.json";

pub async fn process_referrals(ctx: &ServiceContext) -> Result<Referrals> {
    let referrals: Vec<Referral> = File::from_path(FilePath::content(FILE_NAME)).read_as_json().await?;

    Ok(Referrals { referrals })
}
