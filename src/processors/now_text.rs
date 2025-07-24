pub use crate::prelude::*;
use crate::{
    domain::models::now_text::NowText,
    services::{file_service::{File, FilePath}, ServiceContext},
};

const FILE_NAME: &str = "now.md";

pub async fn process_now_text(ctx: &ServiceContext) -> Result<NowText> {
    let text = File::from_path(FilePath::content(FILE_NAME)).read_text().await?;

    Ok(NowText { now_text: text })
}
