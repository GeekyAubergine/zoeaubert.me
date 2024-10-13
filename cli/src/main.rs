#[macro_use]
extern crate lazy_static;

use clap::{Parser, Subcommand};
use commands::upload;
use dotenvy_macro::dotenv;
use error::Error;
use prelude::Result;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use utils::api_client::make_api_client;
use std::path::Path;
use tracing::{debug, info, Level};

pub mod commands;
pub mod error;
pub mod microblog_archive;
pub mod microposts;
pub mod prelude;
pub mod silly_names;
pub mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    Upload,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let api_client = make_api_client().await?;

    let args = Args::parse();

    match args.command {
        Command::Upload => upload::upload(&api_client).await?,
    }

    Ok(())
}

pub fn content_folder_path<'a>() -> &'a Path {
    &Path::new("../content")
}
