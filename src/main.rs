#![allow(unused)]

pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod prelude;
pub mod tasks;

use std::{hash::{DefaultHasher, Hash, Hasher}, path::Path, process::Command, time::Duration};

use application::commands::update_all_data_command::update_all_data_command;
use build_data::BUILD_DATE;
use dircpy::copy_dir;
use domain::{repositories::Profiler, state::State};
use dotenvy_macro::dotenv;
use error::FileSystemError;
use infrastructure::app_state::AppState;
use tasks::render_site::render_site;
use tracing::{info, Level};

use crate::prelude::*;

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

async fn prepare_state() -> Result<AppState> {
    let state = AppState::new().await?;

    update_all_data_command(&state).await?;

    Ok(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Build date: {}", BUILD_DATE);

    let state = prepare_state().await?;

    render_site(&state).await?;

    Ok(())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
