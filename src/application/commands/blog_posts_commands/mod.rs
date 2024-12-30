use serde::{Deserialize, Serialize};

use crate::{error::BlogPostError, prelude::*};

pub mod update_blog_post_command;
pub mod update_blog_posts_command;

pub const BLOG_POSTS_DIR: &str = "blog-posts";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostFileFrontMatter {
    slug: String,
    date: String,
    title: String,
    description: String,
    tags: Vec<String>,
    hero: Option<String>,
    #[serde(rename = "heroAlt")]
    hero_alt: Option<String>,
    #[serde(rename = "heroWidth")]
    hero_width: Option<u32>,
    #[serde(rename = "heroHeight")]
    hero_height: Option<u32>,
}

pub fn front_matter_from_string(s: &str) -> Result<BlogPostFileFrontMatter> {
    serde_yaml::from_str(s).map_err(BlogPostError::unparsable_front_matter)
}
