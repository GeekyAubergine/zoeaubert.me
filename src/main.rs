use crate::prelude::*;

use axum::{routing::get, Router};
use config::load_config;
use error::Error;
use model::{mastodon_post::new_mastodon_posts_repo, AppState};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::Mutex;
use web::web_routes;

mod config;
mod error;
mod model;
mod prelude;
mod web;

async fn load_data_from_file(path: &str) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|_| Error::UnableToFindFile(path.to_string()))?;
    let mut buf_reader = BufReader::new(file);

    let mut data = Vec::new();

    buf_reader
        .read_to_end(&mut data)
        .map_err(|_| Error::UnableToReadFile(path.to_string()))?;

    Ok(data)
}

async fn save_data_to_file(path: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(path).map_err(|_| Error::UnableToWriteFile(path.to_string()))?;

    file.write_all(data)
        .map_err(|_| Error::UnableToWriteFile(path.to_string()))?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let config = load_config().await.unwrap();

    let app_state = AppState::new(config);

    let routes = Router::new().merge(web_routes()).with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
