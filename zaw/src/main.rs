pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod prelude;
pub mod tasks;

use std::{path::Path, process::Command};

use application::commands::update_all_data::update_all_data;
use error::FileSystemError;
use infrastructure::{app_state::AppState, renderers::home_page::render_home_page};
use tracing::Level;

use crate::prelude::*;

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

async fn prepare_folders() -> Result<()> {
    Command::new("rm")
        .arg("-rf")
        .arg("./output")
        .output()
        .expect("Failed to remove output directory");

    Command::new("mkdir")
        .arg("-p")
        .arg("./output/assets")
        .output()
        .expect("Failed to create assets directory");

    Command::new("cp")
        .arg("-r")
        .arg("./assets/")
        .arg("./output/assets/")
        .output()
        .expect("Failed to copy assets");

    Command::new("cp")
        .arg("-r")
        .arg("./_assets/")
        .arg("./output/assets/")
        .output()
        .expect("Failed to copy assets");

    tokio::fs::create_dir_all(Path::new("./output"))
        .await
        .map_err(FileSystemError::create_dir_error)?;

    Ok(())
}

async fn prepare_state() -> Result<AppState> {
    let state = AppState::new().await?;

    update_all_data(&state).await?;

    Ok(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    prepare_folders().await?;
    let state = prepare_state().await?;

    render_home_page(&state).await?;

    Ok(())
}
