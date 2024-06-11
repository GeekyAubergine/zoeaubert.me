#![allow(unused)]
// #![feature(duration_constructors)]
// #[cfg_attr(target_arch = "arm", unstable(feature = "stdarch_aarch32_crc32", issue = "XXXX"))]
// #[cfg_attr(not(target_arch = "arm"), stable(feature = "stdarch_aarch64_crc32", since = "1.80.0"))]
#[macro_use]
extern crate lazy_static;

use std::{fs, path::Path, sync::Arc, thread::sleep, time::Duration};

use crate::{
    infrastructure::bus::{event_queue::make_event_channel, job_runner::make_job_channel, Bus},
    prelude::*,
};

use application::events::Event;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    middleware,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use domain::{
    about::about_listener::AboutListener, blog_posts::blog_posts_listener::BlogPostsListener,
    faq::faq_listener::FaqListener, games::games_listener::GamesListener,
    lego::lego_listener::LegoListener, silly_names::silly_names_listener::SillyNamesListener,
    status_lol::status_lol_listener::StatusLolListener,
};
use error::Error;
use infrastructure::{
    app_state::{AppState, AppStateData},
    bus::logger_listener::LoggerListener,
    cdn::{Cdn, CdnPath},
    config::Config,
};
use prelude::Result;
use routes::router;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{
    sync::{mpsc::channel, RwLock},
    task,
};
use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::ServiceBuilderExt;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer, services::ServeDir};
use tower_livereload::LiveReloadLayer;
use tracing::{debug, info, Level};

mod application;
mod domain;
mod error;
mod infrastructure;
mod prelude;
mod routes;
pub mod utils;

pub const GAMES_ARCHIVE_FILENAME: &str = "games.json";
pub const LEGO_ARCHIVE_FILENAME: &str = "lego.json";
pub const STATUS_LOL_ARCHIVE_FILENAME: &str = "status_lol.json";
pub const ABOUT_ARCHIVE_FILENAME: &str = "about.json";

pub const ONE_HOUR_CACHE_PERIOD: Duration = Duration::new(60 * 60 - 1, 0);
pub const ONE_DAY_CACHE_PERIOD: Duration = Duration::new(60 * 60 * 24 - 1, 0);

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
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting up...");

    let config = load_config().await?;

    prepare_folders(&config).await?;

    let (job_sender, job_receiver) = make_job_channel();
    let (event_sender, event_receiver) = make_event_channel();

    let mut state = AppStateData::new(&config, job_sender, event_sender).await;

    let state = Arc::new(state);

    let mut queue = Bus::new(state.clone(), job_receiver, event_receiver);

    queue.add_event_listener(Box::new(LoggerListener::new()));
    queue.add_event_listener(Box::new(GamesListener::new()));
    queue.add_event_listener(Box::new(StatusLolListener::new()));
    queue.add_event_listener(Box::new(AboutListener::new()));
    queue.add_event_listener(Box::new(FaqListener::new()));
    queue.add_event_listener(Box::new(SillyNamesListener::new()));
    queue.add_event_listener(Box::new(BlogPostsListener::new()));

    println!("Starting jobs...");
    state.dispatch_event(Event::ServerBooted).await?;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let services = ServiceBuilder::new()
        .layer(LiveReloadLayer::new())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(cors);

    let static_files = ServeDir::new("./static");
    let asset_files = ServeDir::new("./_assets");

    let app = router()
        .with_state(state)
        .nest_service("/static", static_files)
        .nest_service("/assets", asset_files)
        .layer(services);

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
