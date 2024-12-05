use chrono::{DateTime, Utc};

pub trait FormatDate {
    fn short_iso(&self) -> String;
    fn datetime(&self) -> String;
    fn without_time(&self) -> String;
    fn month_as_word(&self) -> String;
}

impl FormatDate for DateTime<Utc> {
    fn short_iso(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn datetime(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn without_time(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }

    fn month_as_word(&self) -> String {
        self.format("%B %e, %Y").to_string()
    }
}
