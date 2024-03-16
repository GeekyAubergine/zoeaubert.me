use crate::{
    domain::models::tag::Tag,
    error::Error,
    infrastructure::{config::Config, content_dir::ContentDir},
    prelude::*,
    utils::{find_files_rescurse, parse_date},
};

use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::info;

use crate::domain::models::{blog_post::BlogPost, image::Image};

const BLOG_POSTS_DIR: &str = "blogPosts";

#[derive(Debug, Clone, Deserialize)]
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

fn front_matter_from_string(s: &str) -> Result<BlogPostFileFrontMatter> {
    serde_yaml::from_str(s).map_err(Error::ParseBlogFrontMatter)
}

fn file_to_blog_post(s: &str) -> Result<BlogPost> {
    let split = s.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let content = split.get(2);

    match (front_matter, content) {
        (Some(front_matter), Some(content)) => {
            let front_matter = front_matter_from_string(front_matter)?;

            let tags = front_matter
                .tags
                .iter()
                .map(|tag| Tag::new(tag))
                .collect::<Vec<Tag>>();

            let date = parse_date(front_matter.date.as_str())?;

            let hero_image = match (
                front_matter.hero,
                front_matter.hero_alt,
                front_matter.hero_width,
                front_matter.hero_height,
            ) {
                (Some(url), Some(alt), Some(width), Some(height)) => Some(Image::new(
                    url.as_str(),
                    alt.as_str(),
                    width,
                    height,
                    None,
                    None,
                    Some(date),
                    None,
                )),
                _ => None,
            };

            Ok(BlogPost::new(
                front_matter.slug,
                date,
                front_matter.title,
                front_matter.description,
                tags,
                hero_image,
                content.to_owned().to_owned(),
            ))
        }
        _ => Err(Error::ParseBlogPost("Invalid front matter".to_string())),
    }
}

#[derive(Debug, Clone)]
pub struct BlogPostsRepo {
    blog_posts: Arc<RwLock<HashMap<String, BlogPost>>>,
}

impl BlogPostsRepo {
    pub fn new() -> Self {
        Self {
            blog_posts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn reload(&self, config: &Config, content_dir: &ContentDir) -> Result<()> {
        let blog_posts_files = find_files_rescurse(BLOG_POSTS_DIR, "md", config)?;

        let mut blog_posts = HashMap::new();

        for file in blog_posts_files {
            let file_content = content_dir.read_file(&file, config).await?;

            if let Some(file_content) = file_content {
                let blog_post = file_to_blog_post(&file_content)?;

                blog_posts.insert(blog_post.slug().to_owned(), blog_post);
            }
        }

        let mut blog_posts_ref = self.blog_posts.write().await;

        *blog_posts_ref = blog_posts;

        Ok(())
    }

    pub async fn get_all(&self) -> HashMap<String, BlogPost> {
        self.blog_posts.read().await.clone()
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<BlogPost> {
        self.blog_posts.read().await.get(slug).cloned()
    }

    pub async fn get_all_by_published_date(&self) -> Vec<BlogPost> {
        let mut blog_posts = self.blog_posts.read().await.clone();

        let mut blog_posts = blog_posts
            .drain()
            .map(|(_, blog_post)| blog_post)
            .collect::<Vec<BlogPost>>();

        blog_posts.sort_by_key(|b| std::cmp::Reverse(b.date()));

        blog_posts
    }

    pub async fn get_all_by_tag(&self, tag: &str) -> Vec<BlogPost> {
        let blog_posts = self.blog_posts.read().await.clone();

        blog_posts
            .values()
            .filter(|blog_post| {
                blog_post
                    .tags()
                    .iter()
                    .any(|blog_post_tag| blog_post_tag.tag() == tag)
            })
            .cloned()
            .collect()
    }
}
