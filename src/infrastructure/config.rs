use crate::{domain::models::image::Image, error::Error, prelude::*};

use chrono::{DateTime, Utc};
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
pub struct SiteImage {
    url: String,
    alt: String,
    width: u32,
    height: u32,
    date: i64,
}

impl SiteImage {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn alt(&self) -> &str {
        &self.alt
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn date(&self) -> i64 {
        self.date
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfigNavLink {
    name: String,
    url: String,
    target: String,
    rel: String,
}

impl SiteConfigNavLink {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn rel(&self) -> &str {
        &self.rel
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfigSocialNetworkLink {
    name: String,
    url: String,
    show_in_top_nav: bool,
    show_in_footer: bool,
}

impl SiteConfigSocialNetworkLink {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn show_in_top_nav(&self) -> bool {
        self.show_in_top_nav
    }

    pub fn show_in_footer(&self) -> bool {
        self.show_in_footer
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfig {
    url: String,
    title: String,
    description: String,
    author: String,
    image: SiteImage,
    language: String,
    nav_links: Vec<SiteConfigNavLink>,
    social_links: Vec<SiteConfigSocialNetworkLink>,
}

impl SiteConfig {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn image(&self) -> Image {
        Image::new(
            self.image.url(),
            self.image.alt(),
            self.image.width(),
            self.image.height(),
            None,
            None,
            None,
            None,
        )
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn nav_links(&self) -> &[SiteConfigNavLink] {
        &self.nav_links
    }

    pub fn social_links(&self) -> &[SiteConfigSocialNetworkLink] {
        &self.social_links
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
    site: SiteConfig,
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

    pub fn site(&self) -> &SiteConfig {
        &self.site
    }
}
