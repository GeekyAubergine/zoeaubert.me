pub use crate::prelude::*;
use crate::{
    domain::models::now_text::NowText,
    services::{
        file_service::{FileService, ReadableFile},
        ServiceContext,
    },
};

const FILE_NAME: &str = "now.md";

pub fn process_now_text(ctx: &ServiceContext) -> Result<NowText> {
    let text = FileService::content(FILE_NAME.into()).read_text()?;

    Ok(NowText { now_text: text })
}
