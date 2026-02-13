use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CachePath {
    parent: String,
    file_name: String,
    extension: String,
}

const CACHE_DIR: &str = ".cache";

impl CachePath {
    pub fn from_path_str(path: &str) -> Self {
        let path = path.replace(CACHE_DIR, "");

        Self::from_path(Path::new(&path))
    }

    pub fn from_path(path: &Path) -> Self {
        let extension = path.extension().unwrap().to_str().unwrap().to_string();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let parent = path.parent().unwrap().to_str().unwrap().to_string();

        Self {
            parent,
            file_name,
            extension,
        }
    }

    pub fn from_url(url: &str) -> Self {
        let path = url.split('/').skip(3).collect::<Vec<&str>>().join("/");
        Self::from_path_str(&path)
    }

    pub fn as_path(&self) -> PathBuf {
        PathBuf::from(self.to_string())
    }
}

impl Display for CachePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}.{}", self.parent, self.file_name, self.extension)
    }
}
