use serde::{Deserialize, Serialize};
use url::Url;

use super::{source_post::SourcePost, image::Image, post::Post, slug::Slug};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
pub enum TvShowId {
    Tmdb { id: u32 },
}

impl TvShowId {
    pub fn tmdb(id: u32) -> Self {
        Self::Tmdb { id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TvShow {
    pub title: String,
    pub poster: Image,
    pub id: TvShowId,
    pub link: Url,
}

impl TvShow {
    pub fn slug(&self) -> Slug {
        let title = self
            .title
            .replace('&', "")
            .replace(':', "")
            .replace(' ', "-")
            .to_lowercase();
        Slug::new(&format!("/interests/tv/{}", title))
    }
}

impl PartialEq for TvShow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TvShow {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TvShowReview {
    pub tv_show: TvShow,
    pub seasons: Vec<u8>,
    pub scores: Vec<u8>,
    pub review: String,
    pub source_content: SourcePost,
}

impl TvShowReview {
    pub fn season_text(&self) -> String {
        if self.seasons.len() == 1 {
            format!("S{}", self.seasons[0])
        } else {
            format!(
                "S{}-S{}",
                self.seasons[0],
                self.seasons[self.seasons.len() - 1]
            )
        }
    }

    pub fn score_text(&self) -> String {
        self.scores[0].to_string()
    }
}
