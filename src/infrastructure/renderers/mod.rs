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
pub mod tags_pages;
pub mod micro_post_pages;
pub mod mastodon_post_pages;
pub mod lego_pages;
pub mod interests_page;
pub mod games_pages;

pub async fn render_page_with_template<'p, T>(state: &impl State, page: &Page<'p>, template: T) -> Result<()>
where
    T: askama::Template,
{
    debug!("Rendering page: {}", page.slug.relative_link());

    let rendered = template.render().map_err(TemplateError::render_error)?;

    let path = format!("{}index.html", page.slug.relative_link());

    let path = make_output_file_path(&Path::new(&path));

    write_text_file(&path, &rendered).await?;

    state.profiler().page_generated().await?;

    Ok(())
}
