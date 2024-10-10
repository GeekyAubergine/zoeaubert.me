use serde::{Deserialize, Serialize};

use super::site_config::SITE_CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Slug(String);

impl Slug {
    pub fn new(slug: &str) -> Self {
        let slug = match slug.ends_with("/") {
            true => slug.to_string(),
            false => format!("{}/", slug),
        };

        Self(slug)
    }

    pub fn permalink(&self) -> String {
        format!("{}/{}", SITE_CONFIG.url, self.0)
    }

    pub fn relative_link(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
