use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NavigationLink {
    pub name: String,
    pub url: String,
    pub target: String,
    pub rel: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SocialNetworkLink {
    pub name: String,
    pub url: String,
    pub show_in_top_nav: bool,
    pub show_in_footer: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageImage {
    pub url: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
}

impl PageImage {
    pub fn new(url: &str, alt: &str, width: u32, height: u32) -> Self {
        Self {
            url: url.to_owned(),
            alt: alt.to_owned(),
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageConfig {
    pub url: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub image: PageImage,
    pub language: String,
    pub navigation_links: Vec<NavigationLink>,
    pub social_links: Vec<SocialNetworkLink>,
}

pub static SITE_CONFIG: Lazy<PageConfig> = Lazy::new(|| {
    let contents = fs::read_to_string("./site_config.json").unwrap();

    serde_json::from_str(&contents).unwrap()
});
