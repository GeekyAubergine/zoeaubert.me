use std::path::Path;

use tracing::debug;

use crate::domain::services::FileService;
use crate::domain::{repositories::AboutTextRepo, state::State};

use crate::prelude::*;

const SHORT_TEXT_FILE_NAME: &str = "about_short.md";
const LONG_TEXT_FILE_NAME: &str = "about_long.md";

pub async fn update_about_text_command(state: &impl State) -> Result<()> {
    debug!("Updating about text");

    let short_text = state
        .file_service()
        .read_text_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(SHORT_TEXT_FILE_NAME)),
        )
        .await?;
    let long_text = state
        .file_service()
        .read_text_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(LONG_TEXT_FILE_NAME)),
        )
        .await?;

    state.about_text_repo().commit(short_text, long_text).await
}
