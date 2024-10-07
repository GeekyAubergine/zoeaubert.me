use std::fs;

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::build_data::BUILD_DATE;

use super::tag::Tag;

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

#[derive(Debug, Clone)]
pub struct PagePaginationLabel {
    pub url: String,
    pub label: String,
}

impl PagePaginationLabel {
    pub fn new(url: &str, title: &str) -> Self {
        Self {
            url: url.to_owned(),
            label: title.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PagePagination {
    pub next: Option<PagePaginationLabel>,
    pub previous: Option<PagePaginationLabel>,
}

impl PagePagination {
    pub fn new(next: Option<PagePaginationLabel>, prev: Option<PagePaginationLabel>) -> Self {
        Self {
            next,
            previous: prev,
        }
    }
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

static PAGE_CONFIG: Lazy<PageConfig> = Lazy::new(|| {
    let contents = fs::read_to_string("./site_config.json").unwrap();

    serde_json::from_str(&contents).unwrap()
});

#[derive(Debug, Clone)]
pub struct Page<'a> {
    pub slug: &'a str,
    pub url: String,
    pub title: String,
    pub description: String,
    pub author: &'a str,
    pub image: &'a PageImage,
    pub language: &'a str,
    pub build_date: &'a str,
    pub navigation_links: Vec<NavigationLink>,
    pub social_links: Vec<SocialNetworkLink>,
    pub heading: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub read_time: Option<String>,
    pub tags: Vec<Tag>,
    pub page_pagination: Option<PagePagination>,
}

impl<'a> Page<'a> {
    pub fn new(slug: &'a str, title: Option<&str>, description: Option<&str>) -> Self {
        let heading = title.map(|t| t.to_owned());

        let title = match title {
            Some(t) => format!("{} | {}", t, PAGE_CONFIG.title),
            None => PAGE_CONFIG.title.clone(),
        };

        let description = match description {
            Some(d) => d.to_owned(),
            None => PAGE_CONFIG.description.clone(),
        };

        let url = match slug {
            "" | "/" => PAGE_CONFIG.url.to_owned(),
            _ => format!("{}{}", PAGE_CONFIG.url, slug),
        };

        Self {
            slug,
            url,
            title,
            description,
            author: &PAGE_CONFIG.author,
            image: &PAGE_CONFIG.image,
            language: &PAGE_CONFIG.language,
            build_date: BUILD_DATE,
            navigation_links: PAGE_CONFIG
                .navigation_links
                .iter()
                .map(|link| NavigationLink::from(link.clone()))
                .collect(),
            social_links: PAGE_CONFIG
                .social_links
                .iter()
                .map(|link| SocialNetworkLink::from(link.clone()))
                .collect(),
            heading,
            date: None,
            read_time: None,
            tags: vec![],
            page_pagination: None,
        }
    }

    pub fn with_image(mut self, image: &'a PageImage) -> Self {
        self.image = image;
        self
    }

    pub fn with_date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    pub fn with_read_time(mut self, read_time: &str) -> Self {
        self.read_time = Some(read_time.to_owned());
        self
    }

    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_pagination(mut self, pagination: PagePagination) -> Self {
        self.page_pagination = Some(pagination);
        self
    }

    pub fn hide_heading(&mut self) {
        self.heading = None;
    }

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

    pub fn image(&self) -> &PageImage {
        &self.image
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn build_date(&self) -> &str {
        &self.build_date
    }

    pub fn image_url(&self) -> String {
        if self.image.url.starts_with("http") {
            self.image.url.to_owned()
        } else {
            format!("{}/{}", self.url, self.image.url)
        }
    }

    pub fn image_alt(&self) -> &str {
        &self.image.alt
    }

    pub fn navigation_links(&self) -> &[NavigationLink] {
        &self.navigation_links
    }

    pub fn social_links(&self) -> &[SocialNetworkLink] {
        &self.social_links
    }

    pub fn hide_header(&self) -> bool {
        self.heading.is_none()
    }

    pub fn heading(&self) -> Option<&str> {
        self.heading.as_deref()
    }

    pub fn date(&self) -> Option<&DateTime<Utc>> {
        self.date.as_ref()
    }

    pub fn read_time(&self) -> Option<&str> {
        self.read_time.as_deref()
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn page_pagination(&self) -> Option<&PagePagination> {
        self.page_pagination.as_ref()
    }
}
