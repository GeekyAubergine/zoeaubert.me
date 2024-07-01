use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::models::{microblog_archive::MicroblogArchivePost, tag::Tag}, error::Error, infrastructure::{app_state::AppState, bus::job_runner::Job, services::extract_media::extract_media_from_html}, prelude::Result
};

const MICRO_POSTS_DIR: &str = "microBlogArchive/feed.json";

const TAGS_TO_IGNORE: [&str; 2] = ["status", "photography"];
const SELF_URL: &str = "zoeaubert.me";

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFileItem {
    id: String,
    content_html: String,
    content_text: String,
    date_published: DateTime<Utc>,
    url: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFile {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    items: Vec<ArchiveFileItem>,
}

fn archive_item_to_post(item: ArchiveFileItem) -> Result<Option<MicroblogArchivePost>> {
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

    let tags = tags
        .iter()
        .map(|tag| Tag::from_string(tag))
        .collect::<Vec<Tag>>();

    let slug = item
        .id
        .replace("http://geekyaubergine.micro.blog/", "")
        .replace(".html", "")
        .replace('/', "-");

    let content = item
        .content_text
        .replace("uploads/", "https://cdn.geekyaubergine.com/");

    if content.contains(SELF_URL) {
        return Ok(None);
    }

    let mut post = MicroblogArchivePost::new(slug, item.date_published, content.to_owned(), tags);

    let permalink = post.permalink();
    let date = *post.date();

    post = post.with_media(extract_media_from_html(&content, &permalink, &date));

    Ok(Some(post))
}

#[derive(Debug)]
pub struct LoadMicroblogArchiveJob;

impl LoadMicroblogArchiveJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadMicroblogArchiveJob {
    fn name(&self) -> &str {
        "LoadMicroblogArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading microblog archive");
        let archive_file: String = app_state
            .content_dir()
            .read_file(MICRO_POSTS_DIR, app_state.config())
            .await?;

        let archive_file = serde_json::from_str::<ArchiveFile>(&archive_file)
            .map_err(Error::DeserializeArchive)?;

        for item in archive_file.items {
            if let Some(post) = archive_item_to_post(item)? {
                app_state.microblog_archive_repo().commit(post).await;
            }
        }

        Ok(())
    }
}
