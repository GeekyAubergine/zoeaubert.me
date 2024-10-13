#![allow(unused)]
// #![feature(duration_constructors)]
// #[cfg_attr(target_arch = "arm", unstable(feature = "stdarch_aarch32_crc32", issue = "XXXX"))]
// #[cfg_attr(not(target_arch = "arm"), stable(feature = "stdarch_aarch64_crc32", since = "1.80.0"))]
#[macro_use]
extern crate lazy_static;

use dotenvy_macro::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{fs, future::Future, path::Path, pin::Pin, sync::Arc, thread::sleep, time::Duration};

use crate::{
    infrastructure::bus::{event_queue::make_event_channel, job_runner::make_job_channel, Bus},
    prelude::*,
};

use application::events::Event;
use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Response, StatusCode,
    },
    middleware,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use error::{DatabaseError, Error};
use infrastructure::{
    app_state::{AppState, AppStateData},
    bus::logger_listener::LoggerListener,
    config::Config,
    listeners::register_listeners,
};
use prelude::Result;
use routes::{grpc::build_grpc_service, router};
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

pub const GAMES_ARCHIVE_FILENAME: &str = "games.json";
pub const LEGO_ARCHIVE_FILENAME: &str = "lego.json";
pub const STATUS_LOL_ARCHIVE_FILENAME: &str = "status_lol.json";
pub const ABOUT_ARCHIVE_FILENAME: &str = "about.json";
pub const MASTODON_ARCHIVE_FILENAME: &str = "mastodon.json";

pub const ONE_HOUR_CACHE_PERIOD: Duration = Duration::new(60 * 60 - 1, 0);
pub const ONE_DAY_CACHE_PERIOD: Duration = Duration::new(60 * 60 * 24 - 1, 0);

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

async fn load_config() -> Result<Config> {
    let contents = tokio::fs::read_to_string("../config.json")
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

async fn prepare_database() -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv!("DATABASE_URL"))
        .await
        .map_err(DatabaseError::from_connection_error)
}

struct HybridWebService<Web, Grpc> {
    web: Web,
    grpc: Grpc,
}

impl<Web, Grpc> HybridWebService<Web, Grpc> {
    fn new(web: Web, grpc: Grpc) -> Self {
        Self { web, grpc }
    }
}

enum HybridBody<Web, Grpc> {
    Web(Web),
    Grpc(Grpc),
}

enum HybridFuture<Web, Grpc> {
    Web(Web),
    Grpc(Grpc),
}

// impl<Web, Grpc, WebBody, GrpcBody> Service<Request<Body>> for HybridWebService<Web, Grpc>
// where
//     Web: Service<Request<Body>, Response = Response<WebBody>>,
//     Grpc: Service<Request<Body>, Response = Response<GrpcBody>>,
//     Web::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
//     Grpc::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
// {
//     type Response = Response<HybridBody<WebBody, GrpcBody>>;
//     type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//     type Future = HybridFuture<Web::Future, Grpc::Future>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
//         match self.web.poll_ready(cx) {
//             std::task::Poll::Ready(Ok(())) => match self.grpc.poll_ready(cx) {
//                 std::task::Poll::Ready(Ok(())) => std::task::Poll::Ready(Ok(())),
//                 std::task::Poll::Ready(Err(e)) => std::task::Poll::Ready(Err(e.into())),
//                 std::task::Poll::Pending => std::task::Poll::Pending,
//             },
//             std::task::Poll::Ready(Err(e)) => std::task::Poll::Ready(Err(e.into())),
//             std::task::Poll::Pending => std::task::Poll::Pending,
//         }
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         if req
//             .headers()
//             .get(CONTENT_TYPE)
//             .map(|v| v == "application/grpc")
//             .unwrap_or(false)
//         {
//             HybridFuture::Grpc(self.grpc.call(req))
//         } else {
//             HybridFuture::Web(self.web.call(req))
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Starting up...");

    let config = load_config().await?;

    prepare_folders(&config).await?;

    let database = (prepare_database().await?);

    let (event_sender, event_receiver) = make_event_channel();
    let (job_high_priority_sender, job_high_priority_receiver) = make_job_channel();
    let (job_normal_priority_sender, job_normal_priority_receiver) = make_job_channel();
    let (job_low_priority_sender, job_low_priority_receiver) = make_job_channel();

    let mut state = AppStateData::new(
        database,
        &config,
        event_sender,
        job_high_priority_sender,
        job_normal_priority_sender,
        job_low_priority_sender,
    )
    .await;

    let state = Arc::new(state);

    let bus = Bus::new(
        state.clone(),
        event_receiver,
        job_high_priority_receiver,
        job_normal_priority_receiver,
        job_low_priority_receiver,
    );

    let bus = register_listeners(bus);

    info!("Starting jobs...");
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

    build_grpc_service(&state);

    let app = router()
        .with_state(state)
        .nest_service("/static", static_files)
        .nest_service("/assets", asset_files)
        .layer(services);

    // let app = HybridWebService::new(web_service, gprc_service);

    bus.start().await;

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
