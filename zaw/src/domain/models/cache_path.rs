use std::{fmt::Display, path::Path};

use dotenvy_macro::dotenv;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CachePath(String);

const CACHE_DIR: &str = dotenv!("CACHE_DIR");

impl CachePath {
    pub fn from_str(path: &str) -> Self {
        let path = path.replace(CACHE_DIR, "");

        Self(path)
    }

    pub fn from_path(path: &Path) -> Self {
        Self::from_str(path.to_str().unwrap())
    }

    pub fn from_url(url: &str) -> Self {
        let path = url.split('/').skip(3).collect::<Vec<&str>>().join("/");
        Self::from_str(&path)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_path(&self) -> &Path {
        &Path::new(&self.0)
    }
}

impl Display for CachePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
