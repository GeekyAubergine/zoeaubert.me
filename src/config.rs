use crate::{error::Error, load_data_from_file, prelude::*};

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MastodonConfig {
    account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    mastodon: MastodonConfig,
}

pub type ConfigRepo = Arc<Mutex<Config>>;


pub async fn load_config() -> Result<ConfigRepo> {
    let data = load_data_from_file("config.json").await?;

    let config =
        serde_json::from_slice(&data).map_err(|e| Error::UnableToParseConfigFile(e.to_string()))?;

    Ok(Arc::new(Mutex::new(config)))
}
