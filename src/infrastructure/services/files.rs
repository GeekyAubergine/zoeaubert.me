use std::{fs, path::Path};

use crate::{error::Error, infrastructure::config::Config, prelude::Result};

pub fn find_files_rescurse(path: &str, extension: &str, config: &Config) -> Result<Vec<String>> {
    let path = match path.starts_with(config.content_dir()) {
        true => path.to_string(),
        false => format!("{}/{}", config.content_dir(), path),
    };
    let path = Path::new(&path);

    let mut files = vec![];

    for entry in fs::read_dir(path).map_err(Error::ReadDir)? {
        let entry = entry.map_err(Error::ReadDir)?;
        let path = entry.path();

        if path.is_dir() {
            let children = find_files_rescurse(path.to_str().unwrap(), extension, config)?;

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
