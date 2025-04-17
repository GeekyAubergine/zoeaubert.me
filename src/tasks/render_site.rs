use std::path::Path;
use std::process::Command;

use askama::Template;
use chrono::{DateTime, Utc};
use dircpy::copy_dir;
use tokio::fs::copy;
use tokio::try_join;

use crate::build_data::BUILD_DATE;
use crate::domain::models::data::Data;
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
use crate::infrastructure::renderers::{
    new_rendering_context_from_state, render_pages, RendererContext,
};
use crate::infrastructure::services::page_renderer::PageRenderer;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

use tracing::{error, info};

use crate::{error::SiteBuildError, prelude::*};

const COMPILED_ASSETS_DIR: &str = "./_assets";
const ASSETS_DIR: &str = "./output/assets";

const ROBOTS_INPUT_FILE: &str = "./assets/robots.txt";
const ROBOTS_OUTPUT_FILE: &str = "./output/robots.txt";

async fn prepare_folders(state: &impl State) -> Result<()> {
    Command::new("rm")
        .arg("-rf")
        .arg("./output")
        .output()
        .expect("Failed to remove output directory");

    Command::new("mkdir")
        .arg("-p")
        .arg("./output/assets/.")
        .output()
        .expect("Failed to create assets directory");

    state
        .file_service()
        .copy_dir(Path::new("assets"), Path::new("output/assets"))
        .await?;

    copy_dir("_assets", "output/assets");

    state.file_service().make_dir(Path::new("./output")).await?;

    Ok(())
}

async fn compile_css(state: &impl State) -> Result<()> {
    state
        .file_service()
        .copy_dir(Path::new(COMPILED_ASSETS_DIR), Path::new(ASSETS_DIR))
        .await?;

    Ok(())
}

async fn compile_assets(state: &impl State) -> Result<()> {
    compile_css(state).await?;

    state
        .file_service()
        .copy(
            &Path::new(ROBOTS_INPUT_FILE),
            &Path::new(ROBOTS_OUTPUT_FILE),
        )
        .await?;

    Ok(())
}

async fn read_disallowed_routes_from_robot_file(state: &impl State) -> Result<Vec<String>> {
    let robots_txt = state
        .file_service()
        .read_text_file(Path::new(ROBOTS_OUTPUT_FILE))
        .await?;

    let split_before_blanket_disallow = robots_txt
        .split("User-agent: AdsBot-Google")
        .collect::<Vec<&str>>();

    let disallowed_routes = split_before_blanket_disallow
        .first()
        .unwrap()
        .lines()
        .filter(|line| line.starts_with("Disallow:"))
        .map(|line| line.replace("Disallow: ", ""))
        .collect();

    Ok(disallowed_routes)
}

pub async fn render_site(state: &impl State) -> Result<()> {
    info!("Building site");

    prepare_folders(state).await?;
    compile_assets(state).await?;

    let start = std::time::Instant::now();

    let context: RendererContext = new_rendering_context_from_state(state).await?;

    render_pages(state, &context).await?;

    let disallowed_routes = read_disallowed_routes_from_robot_file(state).await?;

    state
        .page_rendering_service()
        .build_sitemap(state, &disallowed_routes)
        .await?;

    let duration = start.elapsed();

    state
        .profiler()
        .set_page_generation_duration(duration)
        .await?;

    state.page_rendering_service().render_pages(state).await?;

    state.profiler().print_results().await?;

    Ok(())
}
