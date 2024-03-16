
pub trait FormatDate {
    fn short_iso(&self) -> String;
    fn datetime(&self) -> String;
}

impl FormatDate for chrono::DateTime<chrono::Utc> {
    fn short_iso(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }

    fn datetime(&self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}
