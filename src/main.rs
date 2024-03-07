#![allow(unused)]
#![feature(duration_constructors)]

use std::{path::Path, sync::Arc, thread::sleep, time::Duration};

use crate::{
    application::jobs::ReloadAllDataJob,
    infrastructure::{
        bus::{event_queue::make_event_channel, job_runner::make_job_channel, Bus},
        listeners::{archive_listener::ArchiveListener, logger_listener::LoggerListener},
    },
    prelude::*,
};

use application::events::Event;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use error::Error;
use infrastructure::{
    app_state::{AppState, AppStateData},
    cdn::Cdn,
    config::Config,
};
use prelude::Result;
use routes::router;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{
    sync::{mpsc::channel, RwLock},
    task,
};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{debug, info, Level};

mod application;
mod domain;
mod error;
mod infrastructure;
mod prelude;
mod routes;

pub const GAMES_ARCHIVE_FILENAME: &str = "games.json";
pub const LEGO_ARCHIVE_FILENAME: &str = "lego.json";
pub const STATUS_LOL_ARCHIVE_FILENAME: &str = "status_lol.json";
pub const ABOUT_ARCHIVE_FILENAME: &str = "about.json";
pub const FAQ_ARCHIVE_FILENAME: &str = "faq.json";

pub const ONE_HOUR_CACHE_PERIOD: Duration = Duration::from_secs(58 * 60);

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

async fn load_config() -> Result<Config> {
    let contents = tokio::fs::read_to_string("./config.json")
        .await
        .map_err(Error::ReadConfigFile)?;

    Config::from_json(&contents)
}

async fn prepare_folders(config: &Config) -> Result<()> {
    tokio::fs::create_dir_all(config.cache_dir())
        .await
        .map_err(Error::MakeFolder)?;

    tokio::fs::create_dir_all(config.archive_dir())
        .await
        .map_err(Error::MakeFolder)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Starting up...");

    let config = load_config().await?;

    prepare_folders(&config).await?;

    let (job_sender, job_receiver) = make_job_channel();
    let (event_sender, event_receiver) = make_event_channel();

    let mut state = AppStateData::new(&config, job_sender, event_sender).await;

    state.load_from_archive().await?;

    let state = Arc::new(state);

    let mut queue = Bus::new(state.clone(), job_receiver, event_receiver);

    queue.add_event_listener(Box::new(LoggerListener::new()));
    queue.add_event_listener(Box::new(ArchiveListener::new()));

    println!("Starting jobs...");
    state.dispatch_job(ReloadAllDataJob::new()).await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let static_files = ServeDir::new("./static");
    let asset_files = ServeDir::new("./_assets");

    let app = router()
        .with_state(state)
        .nest_service("/static", static_files)
        .nest_service("/assets", asset_files)
        .layer(cors);

    // match cdn
    //     .file_exists(CndPath::new(
    //         "2024/01/14/d7e4347cfe88a444a5ee957cff044ba0.jpeg".to_owned(),
    //     ))
    //     .await
    // {
    //     Ok(exists) => {
    //         println!("File exists: {}", exists);
    //     }
    //     Err(e) => {
    //         println!("Failed to check file: {}", e);
    //     }
    // }

    queue.start().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_json<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    debug!("Making request to: {}", url);

    let resp = reqwest::get(url).await.map_err(Error::HttpReqwest)?;

    let json = resp.json::<T>().await.map_err(Error::HttpReqwest)?;

    Ok(json)
}

async fn save_archive_file<T>(config: &Config, data: &T, filename: &str) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(data).map_err(Error::SerializeArchive)?;

    let path = [config.archive_dir(), "/", filename].concat();

    debug!("Saving archive file: {}", path);

    tokio::fs::write(path, json)
        .await
        .map_err(Error::FileSystemUnreadable)?;

    Ok(())
}

async fn load_archive_file<T>(config: &Config, filename: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let path = [config.archive_dir(), "/", filename].concat();

    debug!("Loading archive file: {}", path);

    let json = tokio::fs::read_to_string(path)
        .await
        .map_err(Error::FileSystemUnreadable)?;

    let data = serde_json::from_str(&json).map_err(Error::DeserializeArchive)?;

    Ok(data)
}
