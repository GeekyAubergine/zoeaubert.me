use std::path::Path;

use crate::prelude::*;

use tracing::debug;

use crate::domain::models::page::Page;
use crate::domain::repositories::Profiler;
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::error::{FileSystemError, TemplateError};

pub mod blog_pages;
pub mod formatters;
pub mod games_pages;
pub mod home_page;
pub mod interests_page;
pub mod lego_pages;
pub mod mastodon_post_pages;
pub mod micro_post_pages;
pub mod movie_pages;
pub mod tags_pages;
pub mod timeline_pages;
pub mod tv_show_pages;
pub mod photo_pages;
pub mod albums_and_photos_renderers;
pub mod years_pages;

pub async fn render_page_with_template<'p, T>(
    state: &impl State,
    page: &Page<'p>,
    template: T,
) -> Result<()>
where
    T: askama::Template,
{
    debug!("Rendering page: {}", page.slug.relative_link());

    let rendered = template.render().map_err(TemplateError::render_error)?;

    let path = format!("{}index.html", page.slug.relative_link());

    let path = state
        .file_service()
        .make_output_file_path(&Path::new(&path));

    state
        .file_service()
        .write_text_file_blocking(&path, &rendered)
        .await?;

    state.profiler().page_generated().await?;

    Ok(())
}


pub async fn render_page_with_template2<'p, T>(
    state: &impl State,
    page: Page<'p>,
    template: T,
) -> Result<()>
where
    T: askama::Template,
{
    debug!("Rendering page: {}", page.slug.relative_link());

    let rendered = template.render().map_err(TemplateError::render_error)?;

    let path = format!("{}index.html", page.slug.relative_link());

    let path = state
        .file_service()
        .make_output_file_path(&Path::new(&path));

    state
        .file_service()
        .write_text_file_blocking(&path, &rendered)
        .await?;

    state.profiler().page_generated().await?;

    Ok(())
}
