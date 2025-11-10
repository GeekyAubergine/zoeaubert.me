use serde::{Deserialize, Serialize};

use crate::domain::models::image::Image;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize, Eq, Hash)]
#[serde(tag = "type")]
pub enum BookID {
    OpenLibrary { id: u32 },
}

impl BookID {
    pub fn as_string(&self) -> String {
        match self {
            Self::OpenLibrary { id } => id.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub cover: Image,
    pub id: BookID,
}
