use std::path::Path;

use basic_pages::render_basic_pages;
use interest_pages::render_interest_pages;
use non_basic_pages::render_non_basic_pages;
use omni_post::render_omni_post_pages;
use tokio::try_join;

use crate::domain::state::State;
use crate::prelude::*;
use tracing::debug;

use crate::domain::models::page::Page;
use crate::domain::repositories::Profiler;
use crate::domain::services::FileService;
use crate::error::{FileSystemError, TemplateError};

pub mod basic_pages;
pub mod formatters;
pub mod interest_pages;
pub mod non_basic_pages;
pub mod omni_post;

pub async fn render_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_basic_pages(state),
        render_interest_pages(state),
        render_non_basic_pages(state),
        render_omni_post_pages(state),
    )?;

    Ok(())
}
