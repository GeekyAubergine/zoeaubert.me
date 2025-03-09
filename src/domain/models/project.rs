use serde::{Deserialize, Serialize};
use url::Url;

use super::{image::Image, slug::Slug};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub image: Image,
    pub rank: u8,
    pub link: String,
    pub original_data_hash: u64,
}
