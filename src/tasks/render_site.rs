use std::path::Path;
use std::process::Command;

use askama::Template;
use chrono::{DateTime, Utc};
use dircpy::copy_dir;
use tokio::fs::copy;
use tokio::try_join;

use crate::build_data::BUILD_DATE;
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

use tracing::{error, info};

use crate::{error::SiteBuildError, prelude::*};

const TAILWIND_INPUT_FILE: &str = "./assets/css/styles.css";
const TAILWIND_INTERMEDIATE_FILE: &str = "./output/assets/css/tw-compiled.css";

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

fn compile_tailwind(input: &str, output: Option<&str>) -> Result<()> {
    let input = Path::new(input);
    let output = output.map(Path::new);
    let tailwind_module_location = Path::new("./node_modules/.bin/tailwindcss");

    let mut command = format!(
        "ENVIRONMENT=production {} -i {}",
        tailwind_module_location.display(),
        input.display(),
    );

    if let Some(output) = output {
        command = format!("{} -o {}", command, output.display());
    }

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => {
                info!("Successfully compiled Tailwind CSS");
                info!(
                    "--------\n{}\n--------",
                    String::from_utf8_lossy(&output.stdout)
                );
                Ok(())
            }
            false => {
                error!("Failed to compile Tailwind CSS");
                error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                Err(SiteBuildError::unable_to_compile_tailwind_css())
            }
        },
        Err(e) => {
            error!("Failed to compile Tailwind CSS");
            error!("Error: {}", e);
            Err(SiteBuildError::unable_to_compile_tailwind_css())
        }
    }
}

fn run_lightning(input: &str, output: &str) -> Result<()> {
    let command = format!(
        "./node_modules/.bin/lightningcss --minify --bundle --targets '>= 0.25%' {} -o {}",
        input, output
    );

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => {
                info!("Successfully compiled Lightning CSS");
                info!(
                    "--------\n{}\n--------",
                    String::from_utf8_lossy(&output.stdout)
                );
                Ok(())
            }
            false => {
                error!("Failed to compile Lightning CSS");
                error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                Err(SiteBuildError::unable_to_compile_lightning_css())
            }
        },
        Err(e) => {
            error!("Failed to compile Lightning CSS");
            error!("Error: {}", e);
            Err(SiteBuildError::unable_to_compile_lightning_css())
        }
    }
}

async fn compile_css(state: &impl State) -> Result<()> {
    state
        .file_service()
        .make_dir(Path::new("./output/assets/css"))
        .await?;

    compile_tailwind(TAILWIND_INPUT_FILE, Some(TAILWIND_INTERMEDIATE_FILE))?;
    run_lightning(
        TAILWIND_INTERMEDIATE_FILE,
        &format!("./output/assets/css/styles-{}.css", BUILD_DATE),
    )?;

    state
        .file_service()
        .delete_file(Path::new(TAILWIND_INTERMEDIATE_FILE))
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

    try_join!(
        render_home_page(state),
        render_basic_pages(state),
        render_post_pages(state),
        render_albums_and_photo_pages(state),
        render_interests_pages(state),
        render_feed_files(state),
    )?;

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
