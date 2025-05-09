#![allow(unused)]

pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod prelude;
pub mod tasks;

pub mod services;

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    process::Command,
    time::Duration,
};

use application::commands::update_all_data_command::update_all_data_command;
use build_data::BUILD_DATE;
use clap::{Parser, Subcommand};
use dircpy::copy_dir;
use domain::{repositories::Profiler, state::State};
use dotenvy_macro::dotenv;
use error::FileSystemError;
use infrastructure::app_state::AppState;
use tasks::{create_content::create_content, render_site::render_site};
use tracing::{info, Level};

use crate::prelude::*;

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

#[derive(Parser)]
#[command(author)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(name = "create", about = "Create new content", alias = "c")]
    Create,
    #[command(name = "build", about = "Build the site", alias = "b")]
    Build,
}

async fn prepare_state() -> Result<AppState> {
    let state = AppState::new().await?;

    update_all_data_command(&state).await?;

    Ok(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    let state = prepare_state().await?;

    match args.command {
        Commands::Create => {
            create_content(&state).await?;
        }
        Commands::Build => {
            info!("Build date: {}", BUILD_DATE);
            render_site(&state).await?;
        }
    }

    Ok(())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
