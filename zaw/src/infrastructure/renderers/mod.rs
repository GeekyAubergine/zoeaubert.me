use std::path::Path;

use tracing::debug;

use crate::domain::models::page::Page;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;
use crate::error::{FileSystemError, TemplateError};

use crate::infrastructure::utils::file_system::{make_output_file_path, write_text_file};
use crate::prelude::*;

pub mod blog_pages;
pub mod formatters;
pub mod home_page;
pub mod timeline_pages;

pub async fn render_page_with_template<'p, T>(state: &impl State, page: &Page<'p>, template: T) -> Result<()>
where
    T: askama::Template,
{
    debug!("Rendering page: {}", page.slug.relative_link());

    let rendered = template.render().map_err(TemplateError::render_error)?;

    let path = make_output_file_path(&format!("{}index.html", page.slug.relative_link()));

    write_text_file(&path, &rendered).await?;

    state.profiler().add_page_generated().await?;

    Ok(())
}
