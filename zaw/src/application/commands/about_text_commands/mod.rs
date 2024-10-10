use crate::{
    domain::{repositories::AboutTextRepo, state::State},
    infrastructure::utils::file_system::{make_content_file_path, read_text_file},
};

use crate::prelude::*;

const SHORT_TEXT_FILE_NAME: &str = "about_short.md";
const LONG_TEXT_FILE_NAME: &str = "about_long.md";

pub async fn update_about_text(state: &impl State) -> Result<()> {
    let short_text = read_text_file(&make_content_file_path(SHORT_TEXT_FILE_NAME)).await?;
    let long_text = read_text_file(&make_content_file_path(LONG_TEXT_FILE_NAME)).await?;

    state.about_text_repo().commit(short_text, long_text).await
}
