pub use crate::prelude::*;
use crate::{
    domain::models::{faq::Faq, now_text::NowText},
    services::{file_service::FilePath, ServiceContext},
};

const FILE_NAME: &str = "faq.md";

pub async fn process_faq(ctx: &ServiceContext) -> Result<Faq> {
    let text = FilePath::content(FILE_NAME).read_text().await?;

    Ok(Faq { faq: text })
}
