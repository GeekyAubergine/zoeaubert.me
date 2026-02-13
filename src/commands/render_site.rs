use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::Utc;

use crate::build_data::BUILD_DATE;
use crate::domain::models::data::Data;

use crate::prelude::*;
use crate::renderer::{RendererContext, new_rendering_context_from_data, render_pages};
use crate::services::file_service::{FileService, ReadableFile};

use tracing::{info, instrument};

fn prepare_folders() -> Result<()> {
    Command::new("rm")
        .arg("-rf")
        .arg("./output")
        .output()
        .expect("Failed to remove output directory");

    Ok(())
}

fn copy_assets() -> Result<()> {
    FileService::copy_dir(
        Path::new("./assets/fonts"),
        Path::new("./output/assets/fonts"),
    )?;
    FileService::copy_dir(Path::new("./assets/img"), Path::new("./output/assets/img"))?;

    FileService::copy(
        Path::new("./assets/css/codestyle.css"),
        Path::new("./output/css/codestyle.css"),
    )?;

    let css_file_name = format!("styles-{}.css", BUILD_DATE);

    FileService::copy(
        Path::new(&format!("./_assets/css/{}", css_file_name)),
        Path::new(&format!("./output/assets/css/{}", css_file_name)),
    )?;

    Ok(())
}

fn read_disallowed_routes_from_robot_file() -> Result<Vec<String>> {
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

#[instrument(skip_all)]
pub fn render_site(data: Data) -> Result<()> {
    info!("Rendering site");

    let start = Utc::now();

    prepare_folders()?;
    copy_assets()?;

    let context: RendererContext = new_rendering_context_from_data(data)?;

    render_pages(&context)?;

    let disallowed_routes = read_disallowed_routes_from_robot_file()?;

    let page_count = context.renderer.build_sitemap(&disallowed_routes)?;

    info!(
        "Rendering site | Pages {} [{}ms]",
        page_count,
        (Utc::now() - start).num_milliseconds()
    );

    Ok(())
}
