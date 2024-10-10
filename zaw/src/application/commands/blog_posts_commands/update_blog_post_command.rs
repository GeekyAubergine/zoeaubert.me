use std::path::Path;

use crate::domain::{models::slug::Slug, queries::blog_post_queries::commit_blog_post};
use crate::infrastructure::utils::date::parse_date;
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::{
    domain::{
        models::{blog_post::BlogPost, image::Image, tag::Tag},
        state::State,
    },
    error::{BlogPostError, Error},
    infrastructure::utils::file_system::read_text_file,
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

async fn file_to_blog_post(state: &impl State, file_contents: &str) -> Result<()> {
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

            let mut post = BlogPost::new(
                Slug::new(&front_matter.slug),
                date,
                front_matter.title,
                front_matter.description,
                tags,
                content.to_owned().to_owned(),
            );

            let permalink = post.permalink();

            // TODO
            // if let (Some(url), Some(alt), Some(width), Some(height)) = (
            //     front_matter.hero,
            //     front_matter.hero_alt,
            //     front_matter.hero_width,
            //     front_matter.hero_height,
            // ) {
            //     post = post.with_hero_image(
            //         Image::new(
            //             &ImageUuid(Uuid::new_v5(&Uuid::NAMESPACE_URL, url.as_bytes())),
            //             url.as_str(),
            //             alt.as_str(),
            //             width,
            //             height,
            //         )
            //         .with_date(date)
            //         .with_parent_permalink(&permalink),
            //     );
            // }

            // post = post.with_media(
            //     extract_media_from_markdown(app_state, content, &permalink, &date).await,
            // );

            commit_blog_post(state, &post).await?;

            Ok(())
        }
        _ => Err(BlogPostError::unparsable_blog_post()),
    }
}

pub async fn update_blog_post_command<P>(state: &impl State, file_path: &P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Display,
{
    info!("Updating blog post: [{}]", file_path);

    let file_contents = read_text_file(file_path).await?;

    file_to_blog_post(state, &file_contents).await
}
