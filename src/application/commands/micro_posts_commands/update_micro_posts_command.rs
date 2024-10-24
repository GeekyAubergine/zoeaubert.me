use std::path::Path;

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::{
        models::{image::Image, media::Media, micro_post::MicroPost, slug::Slug, tag::Tag},
        repositories::{MicroPostsRepo, Profiler},
        services::{CacheService, CdnService, FileService, ImageService},
        state::State,
    },
    error::MicroPostError,
    infrastructure::utils::date::parse_date,
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

async fn process_file(state: &impl State, file_path: &Path, content: &str) -> Result<()> {
    let split = content.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = match content.get(front_matter_len + 6..) {
        Some(content) => Ok(content.to_string()),
        None => Err(MicroPostError::no_content(file_path.to_path_buf())),
    }?;

    let front_matter = match front_matter {
        Some(front_matter) => front_matter_from_string(front_matter),
        None => Err(MicroPostError::no_front_matter(file_path.to_path_buf())),
    }?;

    let date = parse_date(front_matter.date.as_str())?;

    let slug_date = date.format("%Y-%m-%d").to_string();

    let file_name = file_path
        .file_name()
        .ok_or(MicroPostError::invalid_file_name(file_path.to_path_buf()))?
        .to_str()
        .ok_or(MicroPostError::invalid_file_name(file_path.to_path_buf()))?;

    let slug = Slug::new(&format!("micros/{}/{}", slug_date, file_name));

    let last_modified = state
        .file_service()
        .get_file_last_modified(file_path)
        .await?;

    if let Some(existing) = state.micro_posts_repo().find_by_slug(&slug).await? {
        if existing.updated_at == last_modified {
            return Ok(());
        }
    }

    let media = state
        .image_service()
        .find_images_in_markdown(state, &content, &date, &slug)
        .await?
        .iter()
        .map(|i| Media::from(i))
        .collect::<Vec<Media>>();

    let tags = front_matter
        .tags
        .iter()
        .map(|tag| Tag::from_string(tag))
        .collect::<Vec<Tag>>();

    let micro_post = MicroPost::new(slug, date, content.to_string(), media, tags, last_modified);

    state.micro_posts_repo().commit(&micro_post).await?;

    Ok(())
}

pub async fn update_micro_posts(state: &impl State) -> Result<()> {
    info!("Updating micro posts");

    let files = state
        .file_service()
        .find_files_rescurse(
            &state
                .file_service()
                .make_content_file_path(&Path::new(MICRO_POSTS_DIR)),
            "md",
        )
        .await?;

    for file in files {
        state.profiler().post_processed().await?;

        let file = Path::new(&file);
        let content = state.file_service().read_text_file(&file).await?;

        process_file(state, &file, &content).await?;
    }

    Ok(())
}
