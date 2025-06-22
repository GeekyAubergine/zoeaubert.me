use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::{
    domain::models::{media::Media, micro_post::MicroPost, slug::Slug, tag::Tag},
    error::MicroPostError,
    infrastructure::utils::date::parse_date,
    prelude::*,
    services::{file_service::FilePath, ServiceContext},
};
const MICRO_POSTS_DIR: &str = "micros";

pub const MARKDOWN_LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\(https?://[^\s]+\)"#).unwrap());

#[derive(Debug, Clone, Deserialize)]
pub struct MicroPostFrontMatter {
    date: String,
    tags: Vec<String>,
}

fn description_from_string(s: &str) -> Option<String> {
    let lines = s
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let first_line = lines.iter().next()?;

    let first_line = first_line.replace("[", "").replace("]", "");

    let first_line = MARKDOWN_LINK_REGEX.replace_all(&first_line, "");

    if first_line.contains("<") {
        let first_line = first_line.split('<').collect::<Vec<&str>>().join("");
    }

    let sentences = first_line.split('.').collect::<Vec<&str>>();

    let first_sentence = sentences.iter().next()?;

    return Some(first_sentence.to_string());
}

fn front_matter_from_string(s: &str) -> Result<MicroPostFrontMatter> {
    serde_yaml::from_str(s).map_err(MicroPostError::unable_to_parse_front_matter)
}

async fn process_file(
    ctx: &ServiceContext,
    file_path: FilePath,
    content: &str,
) -> Result<MicroPost> {
    let split = content.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = match content.get(front_matter_len + 6..) {
        Some(content) => Ok(content.to_string()),
        None => Err(MicroPostError::no_content(file_path)),
    }?;

    let front_matter = match front_matter {
        Some(front_matter) => front_matter_from_string(front_matter),
        None => Err(MicroPostError::no_front_matter(file_path)),
    }?;

    let date = parse_date(front_matter.date.as_str())?;

    let slug_date = date.format("%Y-%m-%d").to_string();

    let file_name = file_path
        .file_name()
        .ok_or(MicroPostError::invalid_file_name(file_path))?
        .to_str()
        .ok_or(MicroPostError::invalid_file_name(file_path))?
        .replace(".md", "");

    let slug = Slug::new(&format!("micros/{}/{}", slug_date, file_name));

    let media = ctx
        .image
        .find_images_in_markdown(ctx, &content, &date, &slug)
        .await?
        .iter()
        .map(|i| Media::from(i))
        .collect::<Vec<Media>>();

    let tags = front_matter
        .tags
        .iter()
        .map(|tag| Tag::from_string(tag))
        .collect::<Vec<Tag>>();

    let description = description_from_string(&content);

    let micro_post = MicroPost::new(slug, date, content.to_string(), description, media, tags);

    Ok(micro_post)
}

pub async fn process_micro_posts(ctx: &ServiceContext) -> Result<()> {
    let files = FilePath::content(MICRO_POSTS_DIR)
        .find_recurisve_files("md")
        .await?;

    for file in files {
        let content = file.read_text().await?;

        process_file(ctx, file, &content).await?;
    }

    Ok(())
}
