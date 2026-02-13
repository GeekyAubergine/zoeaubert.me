pub use crate::prelude::*;
use crate::{
    domain::models::about_text::AboutText,
    services::file_service::{FileService, ReadableFile},
};

const SHORT_TEXT_FILE_NAME: &str = "about_short.md";
const LONG_TEXT_FILE_NAME: &str = "about_long.md";

pub fn load_about_text() -> Result<AboutText> {
    let short = FileService::content(SHORT_TEXT_FILE_NAME.into()).read_text()?;
    let long = FileService::content(LONG_TEXT_FILE_NAME.into()).read_text()?;

    Ok(AboutText { short, long })
}
