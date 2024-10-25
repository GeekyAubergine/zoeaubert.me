use serde::{Deserialize, Serialize};
use url::Url;

use super::{image::Image, omni_post::OmniPost, slug::Slug};

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

#[derive(Debug, Clone)]
pub struct TvShowReview {
    pub tv_show: TvShow,
    pub scores: Vec<u8>,
    pub review: String,
    pub post: OmniPost,
}
