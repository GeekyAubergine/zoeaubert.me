use serde::Deserialize;
use tracing::info;

use crate::{
    domain::{models::{micro_post::MicroPost, slug::Slug, tag::Tag}, queries::micro_posts_queries::commit_micro_post, repositories::Profiler, state::State},
    error::MicroPostError,
    infrastructure::utils::{date::parse_date, file_system::{find_files_rescurse, make_content_file_path, read_text_file}, image_extractor::extract_images_from_html},
};

use crate::prelude::*;

const MICRO_POSTS_DIR: &str = "micros";

#[derive(Debug, Clone, Deserialize)]
pub struct MicroPostFrontMatter {
    date: String,
    tags: Vec<String>,
}

fn front_matter_from_string(s: &str) -> Result<MicroPostFrontMatter> {
    serde_yaml::from_str(s).map_err(MicroPostError::unable_to_parse_front_matter)
}

async fn file_to_micro_post(file_path: &str, content: &str) -> Result<MicroPost> {
    let split = content.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = match content.get(front_matter_len + 6..) {
        Some(content) => Ok(content.to_string()),
        None => Err(MicroPostError::no_content(file_path.to_string())),
    }?;

    let front_matter = match front_matter {
        Some(front_matter) => front_matter_from_string(front_matter),
        None => Err(MicroPostError::no_front_matter(file_path.to_string())),
    }?;

    let date = parse_date(front_matter.date.as_str())?;

    let slug_date = date.format("%Y-%m-%d").to_string();

    let file_name = file_path
        .split('/')
        .last()
        .ok_or(MicroPostError::invalid_file_path(file_path.to_string()))?;

    let file_name = file_name
        .split('.')
        .next()
        .ok_or(MicroPostError::invalid_file_name(file_path.to_string()))?;

    let slug = Slug::new(&format!("{}/{}", slug_date, file_name));

    let images = extract_images_from_html(&content, &date, &slug);

    let tags = front_matter
        .tags
        .iter()
        .map(|tag| Tag::from_string(tag))
        .collect::<Vec<Tag>>();

    Ok(MicroPost::new(
        slug,
        date,
        content.to_string(),
        images,
        tags,
    ))
}

pub async fn update_micro_posts(state: &impl State) -> Result<()> {
    info!("Uploading micro posts");

    let files = find_files_rescurse(&make_content_file_path(MICRO_POSTS_DIR), "md")?;

    for file in files {
        state.profiler().add_post_processed().await?;

        let content = read_text_file(&file).await?;

        let micro_post = file_to_micro_post(&file, &content).await?;

        commit_micro_post(state, &micro_post).await?;
    }

    Ok(())
}
