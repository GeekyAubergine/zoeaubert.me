use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SillyName {
    pub uuid: Uuid,
    pub name: String,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl SillyName {
    pub fn from_name(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            deleted_at: None,
        }
    }
}
