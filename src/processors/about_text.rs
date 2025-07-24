pub use crate::prelude::*;
use crate::{
    domain::models::{about_text::AboutText, now_text::NowText},
    services::{file_service::{File, FilePath}, ServiceContext},
};

const SHORT_TEXT_FILE_NAME: &str = "about_short.md";
const LONG_TEXT_FILE_NAME: &str = "about_long.md";

pub async fn process_about_text(ctx: &ServiceContext) -> Result<AboutText> {
    let short = File::from_path(FilePath::content(SHORT_TEXT_FILE_NAME)).read_text().await?;
    let long = File::from_path(FilePath::content(LONG_TEXT_FILE_NAME)).read_text().await?;

    Ok(AboutText { short, long })
}
