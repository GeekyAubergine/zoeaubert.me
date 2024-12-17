use serde::{Deserialize, Serialize};
use url::Url;

use super::{content::Content, image::Image, slug::Slug};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookReview {
    pub book: Book,
    pub score: u8,
    pub review: String,
    pub source_content: Content,
}

impl Book {
    pub fn slug(&self) -> Slug {
        Slug::new(&format!("/interests/books/{}", self.id.as_string()))
    }
}
