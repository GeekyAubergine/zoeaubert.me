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

use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;

use crate::error::FileSystemError;
use crate::infrastructure::renderers::albums_and_photos_renderer::render_albums_and_photo_pages;
use crate::infrastructure::renderers::blog_pages_renderer::render_blog_pages;
use crate::infrastructure::renderers::faq_page_renderer::render_faq_page;
use crate::infrastructure::renderers::feed_renderers::render_feed_files;
use crate::infrastructure::renderers::games_pages_renderer::render_games_pages;
use crate::infrastructure::renderers::home_page_renderer::render_home_page;
use crate::infrastructure::renderers::interests_page_renderer::render_interests_list_page;
use crate::infrastructure::renderers::lego_pages_renderer::render_lego_page;
use crate::infrastructure::renderers::mastodon_post_pages_renderers::render_mastodon_post_pages;
use crate::infrastructure::renderers::micro_post_pages_renderers::render_micro_post_pages;
use crate::infrastructure::renderers::movie_pages_renderer::render_movie_pages;
use crate::infrastructure::renderers::now_page_renderer::render_now_page;
use crate::infrastructure::renderers::photo_pages_renderer::render_photos_page;
use crate::infrastructure::renderers::save_pages_renderer::render_save_page;
use crate::infrastructure::renderers::tags_pages_renderer::render_tags_pages;
use crate::infrastructure::renderers::timeline_renderer::render_timeline_page;
use crate::infrastructure::renderers::tv_show_renderer::render_tv_show_pages;
use crate::infrastructure::renderers::years_renderer::render_years_pages;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_site(state: &impl State) -> Result<()> {
    info!("Building site");

    let start = std::time::Instant::now();

    try_join!(
        render_home_page(state),
        // Posts
        render_blog_pages(state),
        render_micro_post_pages(state),
        render_mastodon_post_pages(state),
        // Timeline and tags
        render_timeline_page(state),
        render_tags_pages(state),
        render_photos_page(state),
        render_years_pages(state),
        // Interests
        render_interests_pages(state),
        render_albums_and_photo_pages(state),
        // Single pages
        render_404_page(state),
        render_save_page(state),
        render_faq_page(state),
        render_now_page(state),
        // Feeds
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

async fn render_interests_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_interests_list_page(state),
        render_lego_page(state),
        render_games_pages(state),
        render_movie_pages(state),
    )?;

    render_tv_show_pages(state).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct FourOFourTemplate {
    page: Page,
}

async fn render_404_page(state: &impl State) -> Result<()> {
    let page = Page::new(Slug::new("/404"), None, None);

    state
        .page_rendering_service()
        .add_page(state, page.slug.clone(), FourOFourTemplate { page }, None)
        .await
}
