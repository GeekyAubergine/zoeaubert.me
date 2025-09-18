#![allow(unused)]

pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod prelude;
pub mod processors;
pub mod renderer;
pub mod tasks;
pub mod utils;

pub mod services;

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    process::Command,
    time::Duration,
};

use build_data::BUILD_DATE;
use clap::{Parser, Subcommand};
use dircpy::copy_dir;
use dotenvy_macro::dotenv;
use error::FileSystemError;
use tasks::render_site::render_site;
use tracing::{info, Level};

use crate::{prelude::*, processors::process_data, services::ServiceContext};

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

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    let ctx = ServiceContext::new()?;

    match args.command {
        Commands::Create => {
            // create_content(&state).await?;
        }
        Commands::Build => {
            info!("Build date: {}", BUILD_DATE);
            let data = process_data(&ctx).await?;
            render_site(&ctx, data).await?;
        }
    }

    Ok(())
}
