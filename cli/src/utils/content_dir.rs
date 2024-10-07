use std::{fs, path::Path};

use tracing::{debug, info};

use crate::{
    error::{Error, FileSystemError},
    prelude::*,
};

const CONTENT_DIR: &str = "../content";

pub struct ContentDir;

impl ContentDir {


    pub async fn read_file(path: &str) -> Result<String> {
        let path = match path.starts_with(CONTENT_DIR) {
            true => path.to_string(),
            false => format!("{}/{}", CONTENT_DIR, path),
        };
        let path = Path::new(&path);

        debug!("Reading file: {:?}", path);

        tokio::fs::read_to_string(path)
            .await
            .map_err(FileSystemError::unable_to_read_file)
    }
}
