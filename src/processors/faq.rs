pub use crate::prelude::*;
use crate::{
    domain::models::faq::Faq,
    services::file_service::{FileService, ReadableFile},
};

const FILE_NAME: &str = "faq.md";

pub fn load_faq() -> Result<Faq> {
    let text = FileService::content(FILE_NAME.into()).read_text()?;

    Ok(Faq { faq: text })
}
