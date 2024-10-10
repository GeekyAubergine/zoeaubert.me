use std::path::Path;

use tracing::debug;

use crate::domain::models::page::Page;
use crate::error::{FileSystemError, TemplateError};

use crate::prelude::*;

pub mod blog_pages;
pub mod formatters;
pub mod home_page;

pub async fn render_page_with_template<'p, T>(page: &Page<'p>, template: T) -> Result<()>
where
    T: askama::Template,
{
    debug!("Rendering page: {}", page.slug.relative_link());

    let rendered = template.render().map_err(TemplateError::render_error)?;

    let path = format!("./output/{}/index.html", page.slug.relative_link());

    let path = Path::new(&path);

    tokio::fs::write(path, rendered)
        .await
        .map_err(FileSystemError::write_error)
}
