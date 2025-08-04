use std::path::{Path, PathBuf};
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

// use crate::renderers::basic_pages::render_basic_pages;
// use crate::renderers::formatters::format_date::FormatDate;

use crate::error::FileSystemError;
use crate::prelude::*;
// use crate::renderers::{new_rendering_context_from_data, render_pages, RendererContext};
use crate::services::file_service::{FileService, ReadableFile};
use crate::services::ServiceContext;
use crate::utils::paginator::paginate;

use tracing::{error, info};

use crate::{error::SiteBuildError, prelude::*};

const COMPILED_ASSETS_DIR: &str = "./_assets";
const ASSETS_DIR: &str = "./output/assets";

const ROBOTS_INPUT_FILE: &str = "./assets/robots.txt";
const ROBOTS_OUTPUT_FILE: &str = "./output/robots.txt";

async fn prepare_folders() -> Result<()> {
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

    FileService::copy_dir(Path::new("assets"), Path::new("output/assets"))?;

    copy_dir("_assets", "output/assets");

    Ok(())
}

async fn compile_css() -> Result<()> {
    FileService::copy_dir(Path::new(COMPILED_ASSETS_DIR), Path::new(ASSETS_DIR))?;

    Ok(())
}

async fn compile_assets() -> Result<()> {
    compile_css().await?;

    FileService::copy(
        &Path::new(ROBOTS_INPUT_FILE),
        &Path::new(ROBOTS_OUTPUT_FILE),
    )?;

    Ok(())
}

async fn read_disallowed_routes_from_robot_file() -> Result<Vec<String>> {
    let robots_txt = FileService::asset(PathBuf::from("robots.txt")).read_text()?;

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

pub async fn render_site(ctx: &ServiceContext, data: Data) -> Result<()> {
    info!("Building site");

    prepare_folders().await?;
    compile_assets().await?;

    let start = std::time::Instant::now();

    // let context: RendererContext = new_rendering_context_from_data(data).await?;

    // render_pages(&context).await?;

    // let disallowed_routes = read_disallowed_routes_from_robot_file().await?;

    // context.renderer.build_sitemap(&disallowed_routes).await?;

    Ok(())
}
