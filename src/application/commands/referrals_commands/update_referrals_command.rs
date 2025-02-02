use std::path::Path;

use tracing::debug;

use crate::domain::models::referral::Referral;
use crate::domain::repositories::ReferralsRepo;
use crate::domain::services::FileService;
use crate::domain::{repositories::AboutTextRepo, state::State};

use crate::prelude::*;

const FILE_NAME: &str = "referrals.json";

pub async fn update_referrals_command(state: &impl State) -> Result<()> {
    debug!("Updating referrals");

    let referrals: Vec<Referral> = state
        .file_service()
        .read_json_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(FILE_NAME)),
        )
        .await?;

    state.referrals_repo().commit(referrals).await
}
