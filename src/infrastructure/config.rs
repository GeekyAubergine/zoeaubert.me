use crate::{error::Error, prelude::*};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigMastodon {
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "clientKey")]
    client_key: String,
    #[serde(rename = "clientSecret")]
    client_secret: String,
    #[serde(rename = "accessToken")]
    access_token: String,
}

impl ConfigMastodon {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn client_key(&self) -> &str {
        &self.client_key
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigStatusLol {
    url: String,
}

impl ConfigStatusLol {
    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigBrickset {
    #[serde(rename = "apiKey")]
    api_key: String,
    username: String,
    password: String,
}

impl ConfigBrickset {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigR2 {
    url: String,
    id: String,
    endpoint: String,
    key: String,
    secret: String,
    bucket: String,
}

impl ConfigR2 {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSteam {
    #[serde(rename = "apiKey")]
    api_key: String,
    #[serde(rename = "steamId")]
    steam_id: String,
}

impl ConfigSteam {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn steam_id(&self) -> &str {
        &self.steam_id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "cacheDir")]
    cache_dir: String,
    #[serde(rename = "archiveDir")]
    archive_dir: String,
    mastodon: ConfigMastodon,
    #[serde(rename = "statusLol")]
    status_lol: ConfigStatusLol,
    brickset: ConfigBrickset,
    r2: ConfigR2,
    steam: ConfigSteam,
}

impl Config {
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Error::ParseConfigFile)
    }

    pub fn cache_dir(&self) -> &str {
        &self.cache_dir
    }

    pub fn archive_dir(&self) -> &str {
        &self.archive_dir
    }

    pub fn mastodon(&self) -> &ConfigMastodon {
        &self.mastodon
    }

    pub fn status_lol(&self) -> &ConfigStatusLol {
        &self.status_lol
    }

    pub fn brickset(&self) -> &ConfigBrickset {
        &self.brickset
    }

    pub fn r2(&self) -> &ConfigR2 {
        &self.r2
    }

    pub fn steam(&self) -> &ConfigSteam {
        &self.steam
    }
}
