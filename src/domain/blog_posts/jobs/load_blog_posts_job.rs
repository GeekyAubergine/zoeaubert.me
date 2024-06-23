use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    application::events::Event,
    domain::{
        blog_posts::blog_post_models::BlogPost,
        models::{
            media::{image::Image, Media},
            tag::Tag,
        },
    },
    error::Error,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
    },
    prelude::Result,
    utils::{extract_media::extract_media_from_markdown, find_files_rescurse, parse_date},
};

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

async fn file_to_blog_post(app_state: &AppState, s: &str) -> Result<BlogPost> {
    let split = s.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = s.get(front_matter_len + 6..);

    match (front_matter, content) {
        (Some(front_matter), Some(content)) => {
            let front_matter = front_matter_from_string(front_matter)?;

            let tags = front_matter
                .tags
                .iter()
                .map(|tag| Tag::from_string(tag))
                .collect::<Vec<Tag>>();

            let date = parse_date(front_matter.date.as_str())?;

            let mut post = BlogPost::new(
                front_matter.slug,
                date,
                front_matter.title,
                front_matter.description,
                tags,
                content.to_owned().to_owned(),
            );

            let permalink = post.permalink();

            if let (Some(url), Some(alt), Some(width), Some(height)) = (
                front_matter.hero,
                front_matter.hero_alt,
                front_matter.hero_width,
                front_matter.hero_height,
            ) {
                post = post.with_hero_image(
                    Image::new(url.as_str(), alt.as_str(), width, height)
                        .with_date(date)
                        .with_parent_permalink(&permalink),
                );
            }

            post = post.with_media(
                extract_media_from_markdown(app_state, content, &permalink, &date).await,
            );

            Ok(post)
        }
        _ => Err(Error::ParseBlogPost("Invalid front matter".to_string())),
    }
}

#[derive(Debug)]
pub struct LoadBlogPostsJob;

impl LoadBlogPostsJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadBlogPostsJob {
    fn name(&self) -> &str {
        "LoadBlogPostsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let blog_posts_files = find_files_rescurse(BLOG_POSTS_DIR, "md", app_state.config())?;

        for file in blog_posts_files {
            let file_content = app_state
                .content_dir()
                .read_file(&file, app_state.config())
                .await?;

            let blog_post = file_to_blog_post(&app_state, &file_content).await?;

            app_state.blog_posts_repo().commit(blog_post).await;
        }

        app_state
            .dispatch_event(Event::BlogPostsRepoUpdated)
            .await?;

        Ok(())
    }
}
