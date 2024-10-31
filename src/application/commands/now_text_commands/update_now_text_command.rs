use std::path::Path;

use tracing::debug;

use crate::domain::repositories::{FaqRepo, NowTextRepo};
use crate::domain::services::FileService;
use crate::domain::{repositories::AboutTextRepo, state::State};

use crate::prelude::*;

const FILE_NAME: &str = "now.md";

pub async fn update_now_text_command(state: &impl State) -> Result<()> {
    debug!("Updating about text");

    let faq = state
        .file_service()
        .read_text_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(FILE_NAME)),
        )
        .await?;

    state.now_text_repo().commit(faq).await
}
