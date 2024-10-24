use std::path::Path;

use crate::domain::models::slug::Slug;
use crate::domain::repositories::{BlogPostsRepo, Profiler};
use crate::domain::services::{FileService, ImageService};
use crate::infrastructure::utils::date::parse_date;
use serde::Deserialize;
use tracing::{debug, info};
use url::Url;

use crate::{
    domain::{
        models::{blog_post::BlogPost, image::Image, tag::Tag},
        state::State,
    },
    error::{BlogPostError, Error},
    prelude::*,
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
    serde_yaml::from_str(s).map_err(BlogPostError::unparsable_front_matter)
}

pub async fn update_blog_post_command(state: &impl State, file_path: &Path) -> Result<()> {
    let file_contents = state.file_service().read_text_file(file_path).await?;

    let split = file_contents.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = file_contents.get(front_matter_len + 6..);

    match (front_matter, content) {
        (Some(front_matter), Some(content)) => {
            let front_matter = front_matter_from_string(front_matter)?;

            let tags = front_matter
                .tags
                .iter()
                .map(|tag| Tag::from_string(tag))
                .collect::<Vec<Tag>>();

            let date = parse_date(front_matter.date.as_str())?;

            let slug = Slug::new(&format!("/blog/{}", front_matter.slug));

            let last_mofied = state
                .file_service()
                .get_file_last_modified(file_path)
                .await?;

            if let Some(existing) = state.blog_posts_repo().find_by_slug(&slug).await? {
                if last_mofied == existing.updated_at {
                    return Ok(());
                }
            }

            info!("Updating blog post: [{:?}]", slug);

            let mut post = BlogPost::new(
                slug.clone(),
                date,
                front_matter.title,
                front_matter.description,
                tags,
                content.to_owned().to_owned(),
                last_mofied,
            );

            if let (Some(url), Some(alt), Some(width), Some(height)) = (
                front_matter.hero,
                front_matter.hero_alt,
                front_matter.hero_width,
                front_matter.hero_height,
            ) {
                let url: Url = url.parse().unwrap();

                let path = url.path();

                let path = Path::new(&path);

                let image = state
                    .image_service()
                    .copy_image_from_url(state, &url, &path, &alt)
                    .await?
                    .with_date(&date)
                    .with_parent_slug(&slug);

                post = post.with_hero_image(image);
            }

            post = post.with_images(
                state
                    .image_service()
                    .find_images_in_markdown(state, content, &date, &slug)
                    .await?,
            );

            state.blog_posts_repo().commit(&post).await?;

            state.profiler().post_processed().await?;

            Ok(())
        }
        _ => Err(BlogPostError::unparsable_blog_post()),
    }
}
