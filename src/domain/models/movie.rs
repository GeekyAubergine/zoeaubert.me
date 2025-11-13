use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::models::{image::Image, slug::Slug};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum MovieId {
    Tmdb { id: u32 },
}

impl MovieId {
    pub fn tmdb(id: u32) -> Self {
        Self::Tmdb { id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Movie {
    pub title: String,
    pub year: u16,
    pub poster: Image,
    pub id: MovieId,
    pub link: Url,
}

impl Movie {
    pub fn slug(&self) -> Slug {
        let title = self
            .title
            .replace('&', "")
            .replace(':', "")
            .replace(' ', "-")
            .to_lowercase();
        Slug::new(&format!("/interests/movies/{}-{}", title, self.year))
    }
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Movie {}
