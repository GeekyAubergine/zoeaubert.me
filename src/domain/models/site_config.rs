use std::fs;

use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;
use dotenvy_macro::dotenv;

use super::image::Image;

#[derive(Debug, Clone, Deserialize)]
pub struct HeaderLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageLinkGroup {
    pub name: String,
    pub links: Vec<PageLink>,
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
            url: url.to_string(),
            alt: alt.to_owned(),
            width,
            height,
        }
    }
}

impl From<Image> for PageImage {
    fn from(image: Image) -> Self {
        Self {
            url: image.cdn_url().as_str().to_string(),
            alt: image.alt,
            width: image.dimensions.width,
            height: image.dimensions.height,
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
    pub header_links: Vec<HeaderLink>,
    pub page_links: Vec<PageLinkGroup>,
    pub social_links: Vec<SocialNetworkLink>,
}

pub static SITE_CONFIG: Lazy<PageConfig> = Lazy::new(|| {
    let contents = fs::read_to_string(dotenv!("SITE_CONFIG")).unwrap();

    serde_json::from_str(&contents).unwrap()
});
