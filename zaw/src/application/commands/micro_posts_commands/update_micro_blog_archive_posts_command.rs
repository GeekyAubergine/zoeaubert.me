use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::{
        models::{micro_post::MicroPost, slug::Slug, tag::Tag},
        queries::micro_posts_queries::commit_micro_post,
        repositories::Profiler,
        state::State,
    },
    infrastructure::utils::{
        file_system::{make_content_file_path, read_json_file},
        image_extractor::extract_images_from_html,
    },
    prelude::*,
};

const MICRO_POSTS_DIR: &str = "microBlogArchive/feed.json";

const TAGS_TO_IGNORE: [&str; 2] = ["status", "photography"];
const SELF_URL: &str = "zoeaubert.me";

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFileItem {
    id: String,
    // content_html: String,
    content_text: String,
    date_published: DateTime<Utc>,
    // url: String,
    tags: Option<Vec<String>>,
}

impl ArchiveFileItem {
    fn tags_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.tags
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFile {
    // version: String,
    // title: String,
    // home_page_url: String,
    // feed_url: String,
    items: Vec<ArchiveFileItem>,
}

fn archive_item_to_post(item: ArchiveFileItem) -> Result<Option<MicroPost>> {
    let tags: Vec<String> = match item.tags {
        Some(tags) => tags,
        None => vec![],
    };

    if tags
        .iter()
        .any(|tag| TAGS_TO_IGNORE.contains(&tag.to_lowercase().as_str()))
    {
        return Ok(None);
    }

    let tags = tags.iter().map(|t| Tag::from_string(t)).collect();

    let id = item
        .id
        .split('/')
        .skip(6)
        .collect::<Vec<&str>>()
        .join("/")
        .replace(".html", "");

    let slug_date = item.date_published.format("%Y/%m/%d").to_string();

    let slug = format!(
        "micros/{}/{}",
        slug_date,
        id
    );

    let slug = Slug::new(&slug);

    let content = item
        .content_text
        .replace("uploads/", "https://cdn.geekyaubergine.com/");

    if content.contains(SELF_URL) {
        return Ok(None);
    }

    let images = extract_images_from_html(&content, &item.date_published, &slug);

    Ok(Some(MicroPost::new(
        slug.clone(),
        item.date_published,
        content,
        images,
        tags,
    )))
}

pub async fn update_micro_blog_archive_posts_command(state: &impl State) -> Result<()> {
    info!("Updating micro blog archive posts");

    let archive_file: ArchiveFile =
        read_json_file(&make_content_file_path(MICRO_POSTS_DIR)).await?;

    for item in archive_file.items {
        state.profiler().add_post_processed().await?;
        if let Some(post) = archive_item_to_post(item)? {
            commit_micro_post(state, &post).await?;
        }
    }

    Ok(())
}
