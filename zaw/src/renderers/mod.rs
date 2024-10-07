use std::path::Path;

use tracing::debug;

use crate::error::FileSystemError;
use crate::models::page::Page;

use crate::prelude::*;

pub mod formatters;
pub mod home_page;

pub async fn render_page_to_file<'p>(page: &Page<'p>, content: &str) -> Result<()> {
    let path = format!("{}{}", "./output", page.slug);

    let path = match path.ends_with('/') {
        true => format!("{}index.html", path),
        false => format!("{}/index.html", path)
    };

    debug!("Rendering page to file: {}", path);

    let path = Path::new(&path);

    tokio::fs::write(path, content)
        .await
        .map_err(FileSystemError::write_error)
}
