pub use crate::prelude::*;
use crate::{
    domain::models::now_text::NowText,
    services::file_service::{FileService, ReadableFile},
};

const FILE_NAME: &str = "now.md";

pub fn load_now_text() -> Result<NowText> {
    let text = FileService::content(FILE_NAME.into()).read_text()?;

    Ok(NowText { now_text: text })
}
