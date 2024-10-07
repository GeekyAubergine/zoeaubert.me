use std::{fs::read_dir, path::Path};

use dotenvy_macro::dotenv;
use tracing::debug;

use crate::error::FileSystemError;

use crate::prelude::Result;

const CONTENT_DIR: &str = dotenv!("CONTENT_DIR");

fn fix_content_dir(path: &str) -> String {
    match path.starts_with(CONTENT_DIR) {
        true => path.to_string(),
        false => format!("{}/{}", CONTENT_DIR, path),
    }
}

pub fn find_files_rescurse(path: &str, extension: &str) -> Result<Vec<String>> {
    let path = fix_content_dir(path);
    let path = Path::new(&path);

    let mut files = vec![];

    for entry in read_dir(path).map_err(FileSystemError::unable_to_read_dir)? {
        let entry = entry.map_err(FileSystemError::unable_to_read_dir)?;
        let path = entry.path();

        if path.is_dir() {
            let children = find_files_rescurse(path.to_str().unwrap(), extension)?;

            for child in children {
                files.push(child);
            }
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    Ok(files)
}

pub async fn read_file_from_content_dir(path: &str) -> Result<String> {
    let path = fix_content_dir(path);
    let path = Path::new(&path);

    debug!("Reading file: {:?}", path);

    tokio::fs::read_to_string(path)
        .await
        .map_err(FileSystemError::unable_to_read_file)
}
