use std::{fs, path::Path};

use crate::{error::Error, prelude::*};

pub mod format_date;
pub mod format_markdown;
pub mod format_number;
pub mod archive;
pub mod extract_media;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
pub use format_date::FormatDate;
pub use format_markdown::FormatMarkdown;
pub use format_number::FormatNumber;
use tracing::debug;

use crate::infrastructure::config::Config;

const DATETIME_FORMAT_1: &str = "%Y-%m-%dT%H:%M:%S";
const DATETIME_FORMAT_2: &str = "%Y-%m-%d %H:%M";
const DATETIME_FORMAT_3: &str = "%Y-%m-%dT%H:%M";
const DATETIME_FORMAT_4: &str = "%Y-%m-%d";

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

pub fn parse_date(s: &str) -> Result<DateTime<Utc>> {
    if let Ok(date) = DateTime::parse_from_rfc3339(s) {
        return Ok(date.with_timezone(&Utc));
    }

    if let Ok(date) = NaiveDateTime::parse_from_str(s, DATETIME_FORMAT_1) {
        return Ok(date.and_utc());
    }

    if let Ok(date) = NaiveDateTime::parse_from_str(s, DATETIME_FORMAT_2) {
        return Ok(date.and_utc());
    }

    if let Ok(date) = NaiveDateTime::parse_from_str(s, DATETIME_FORMAT_3) {
        return Ok(date.and_utc());
    }

    if let Ok(date) = NaiveDate::parse_from_str(s, DATETIME_FORMAT_4) {
        return Ok(date
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc());
    }

    println!("Failed to parse date: {}", s);

    Err(Error::ParseDate(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = parse_date("2022-11-15").unwrap();
        assert_eq!(date.to_rfc3339(), "2022-11-15T00:00:00+00:00");

        let date = parse_date("2022-11-15T19:10").unwrap();
        assert_eq!(date.to_rfc3339(), "2022-11-15T19:10:00+00:00");
    }
}
