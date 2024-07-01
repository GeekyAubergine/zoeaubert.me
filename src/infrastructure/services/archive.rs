use std::{fs, path::Path, sync::Arc, thread::sleep, time::Duration};

use crate::{
    error::Error, infrastructure::{bus::{event_queue::make_event_channel, job_runner::make_job_channel, Bus}, config::Config}, prelude::*
};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    }, middleware, routing::{get, post}, Json, Router
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{
    sync::{mpsc::channel, RwLock},
    task,
};
use tracing::debug;

pub async fn save_archive_file<T>(config: &Config, data: &T, filename: &str) -> Result<()>
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

pub async fn load_archive_file<T>(config: &Config, filename: &str) -> Result<T>
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
