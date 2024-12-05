use std::path::Path;

use askama::Template;
use chrono::{DateTime, Utc};
use tokio::fs::copy;
use tokio::try_join;
use tracing::info;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::queries::omni_post_queries::{
    find_all_omni_posts, find_all_omni_posts_by_tag, OmniPostFilterFlags,
};
use crate::domain::queries::tags_queries::find_tag_counts;
use crate::domain::repositories::Profiler;
use crate::domain::services::{FileService, PageRenderingService};
use crate::domain::state::State;

use crate::infrastructure::renderers::basic_pages::render_basic_pages;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;

use crate::error::FileSystemError;
use crate::infrastructure::renderers::album_and_photo_pages::render_albums_and_photo_pages;
use crate::infrastructure::renderers::feed_page_renderers::render_feed_files;
use crate::infrastructure::renderers::home_pages_renderer::render_home_page;
use crate::infrastructure::renderers::interest_pages::render_interests_pages;
use crate::infrastructure::renderers::post_pages::render_post_pages;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_site(state: &impl State) -> Result<()> {
    info!("Building site");

    let start = std::time::Instant::now();

    try_join!(
        render_home_page(state),
        render_basic_pages(state),
        render_post_pages(state),
        render_albums_and_photo_pages(state),
        render_interests_pages(state),
        render_feed_files(state),
    )?;

    copy(
        state
            .file_service()
            .make_output_file_path(&Path::new("assets/robots.txt")),
        state
            .file_service()
            .make_output_file_path(&Path::new("robots.txt")),
    )
    .await
    .map_err(FileSystemError::copy_file_error)?;

    state.page_rendering_service().build_sitemap(state).await?;

    let duration = start.elapsed();

    state
        .profiler()
        .set_page_generation_duration(duration)
        .await?;

    state.page_rendering_service().render_pages(state).await?;

    state.profiler().print_results().await?;

    Ok(())
}
