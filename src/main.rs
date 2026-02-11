#![allow(unused)]

pub mod domain;
pub mod error;
pub mod prelude;
pub mod processors;
pub mod renderer;
pub mod commands;
pub mod utils;
pub mod config;

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
use commands::render_site::render_site;
use tracing::{info, Level};
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

fn main() -> Result<()> {
    let filter = EnvFilter::new("info,zoeaubert_website=info");

    let file = rolling::daily("logs", "logs.json");
    let (file_writer, _guard) = tracing_appender::non_blocking(file);

    let json = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(file_writer);

    let console = tracing_subscriber::fmt::layer().pretty().with_target(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(json)
        .with(console)
        .init();

    let args = Args::parse();

    let ctx = ServiceContext::new()?;

    match args.command {
        Commands::Create => {
            // create_content(&state).await?;
        }
        Commands::Build => {
            info!("Build date: {}", BUILD_DATE);
            let data = process_data(&ctx)?;
            render_site(&ctx, data)?;
        }
    }

    Ok(())
}
