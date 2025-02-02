use serde::{Deserialize, Serialize};
use url::Url;

use super::{image::Image, slug::Slug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub slug: Slug,
    pub name: String,
    pub description: String,
    pub image: Image,
    pub rank: u8,
    pub link: Url,
    pub original_data_hash: u64,
}
