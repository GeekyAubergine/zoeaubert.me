pub mod error;
pub mod models;
pub mod prelude;
pub mod renderers;

use std::{path::Path, process::Command};

use renderers::home_page::render_home_page;
use tracing::Level;

use crate::prelude::*;

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

#[tokio::main]
async fn main() -> Result<()> {
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

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    tokio::fs::create_dir_all(Path::new("./output"))
        .await
        .unwrap();

    render_home_page().await?;

    Ok(())
}
