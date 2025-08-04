pub use crate::prelude::*;
use crate::{
    domain::models::{faq::Faq, now_text::NowText},
    services::{
        file_service::{FileService, ReadableFile},
        ServiceContext,
    },
};

const FILE_NAME: &str = "faq.md";

pub async fn process_faq(ctx: &ServiceContext) -> Result<Faq> {
    let text = FileService::content(FILE_NAME.into()).read_text()?;

    Ok(Faq { faq: text })
}
