use std::path::Path;

use tracing::debug;

use crate::domain::repositories::FaqRepo;
use crate::domain::services::FileService;
use crate::domain::{repositories::AboutTextRepo, state::State};

use crate::prelude::*;

const FILE_NAME: &str = "faq.md";

pub async fn update_faq_command(state: &impl State) -> Result<()> {
    debug!("Updating about text");

    let faq = state
        .file_service()
        .read_text_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(FILE_NAME)),
        )
        .await?;

    state.faq_repo().commit(faq).await
}
