use std::{collections::VecDeque, ops::Deref};

use serde::{Deserialize, Serialize};

use super::site_config::SITE_CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Slug(String);

impl Slug {
    pub fn new(slug: &str) -> Self {
        let slug = match (slug.starts_with("/"), slug.ends_with("/")) {
            (true, true) => slug.to_owned(),
            (true, false) => format!("{}/", slug),
            (false, true) => format!("/{}", slug),
            (false, false) => format!("/{}/", slug),
        };

        let slug = match slug.starts_with("http") {
            true => slug.split("/").skip(3).collect::<Vec<&str>>().join("/"),
            false => slug,
        };

        Self(slug.replace("//", "/"))
    }

    pub fn permalink(&self) -> String {
        if self.0.starts_with("http") {
            return self.0.clone();
        }
        format!("{}{}", SITE_CONFIG.url, self.0)
    }

    pub fn relative_link(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn append(&self, suffix: &str) -> Self {
        let slug = format!("{}{}", self.0, suffix).replace("//", "/");
        Self::new(&slug)
    }
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
