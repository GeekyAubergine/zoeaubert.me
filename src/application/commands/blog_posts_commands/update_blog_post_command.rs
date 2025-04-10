use std::path::Path;

use crate::application::commands::blog_posts_commands::front_matter_from_string;
use crate::calculate_hash;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::{BlogPostsRepo, Profiler};
use crate::domain::services::{FileService, ImageService};
use crate::infrastructure::utils::date::parse_date;
use chrono::Utc;
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

pub async fn update_blog_post_command(state: &impl State, file_path: &Path) -> Result<()> {
    let file_contents = state.file_service().read_text_file(file_path).await?;

    let hash = calculate_hash(&file_contents);

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

            if let Some(existing) = state.blog_posts_repo().find_by_slug(&slug).await? {
                if hash == existing.original_data_hash {
                    return Ok(());
                }
            }

            info!("Updating blog post: {:?}", slug);

            let mut post = BlogPost::new(
                slug.clone(),
                date,
                front_matter.title,
                front_matter.description,
                tags,
                content.to_owned().to_owned(),
                Utc::now(),
                hash,
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

            state.profiler().entity_processed().await?;

            Ok(())
        }
        _ => Err(BlogPostError::unparsable_blog_post()),
    }
}
