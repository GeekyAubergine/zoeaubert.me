use std::{fs, path::Path};

use crate::{error::Error, prelude::*};


use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use tracing::debug;

use crate::infrastructure::config::Config;

const DATETIME_FORMAT_1: &str = "%Y-%m-%dT%H:%M:%S";
const DATETIME_FORMAT_2: &str = "%Y-%m-%d %H:%M";
const DATETIME_FORMAT_3: &str = "%Y-%m-%dT%H:%M";
const DATETIME_FORMAT_4: &str = "%Y-%m-%d";

pub trait FormatDate {
    fn short_iso(&self) -> String;
    fn datetime(&self) -> String;
}

impl FormatDate for chrono::DateTime<chrono::Utc> {
    fn short_iso(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn datetime(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }
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
