use crate::{
    error::TonicError, microblog_archive::error::MicroBlogArchiveError, prelude::*, utils::{
        api_client::ApiClient, content_dir::ContentDir, image_extraction::extract_images_from_html,
    }
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use shared::zoeaubert_proto::webserver::{MicroPost, UpdateMicroPostRequest};
use tonic::Request;
use tracing::{debug, error, info};
use uuid::Uuid;

const MICRO_POSTS_DIR: &str = "microBlogArchive/feed.json";

const TAGS_TO_IGNORE: [&str; 2] = ["status", "photography"];
const SELF_URL: &str = "zoeaubert.me";

const NAMESPACE: Uuid = Uuid::from_u128(0x0d2f1b7e3248239872984);

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

    let images = extract_images_from_html(&content, &item.date_published);

    Ok(Some(MicroPost {
        uuid: Uuid::new_v5(&NAMESPACE, item.id.as_bytes()).into(),
        slug: slug.clone(),
        date: item.date_published.to_rfc3339(),
        content,
        tags,
        images,
    }))
}

pub async fn upload_microblob_archive_posts(api_client: &ApiClient) -> Result<()> {
    info!("Loading microblog archive");
    let archive_file: String = ContentDir::read_file(MICRO_POSTS_DIR).await?;

    let archive_file = serde_json::from_str::<ArchiveFile>(&archive_file)
        .map_err(MicroBlogArchiveError::unable_to_parse_file)?;

    for item in archive_file.items {
        match archive_item_to_post(item) {
            Ok(Some(post)) => {
                info!("Uploading post: {}", post.slug);

                let request = Request::new(UpdateMicroPostRequest {
                    micro_post: Some(post),
                });

                api_client
                    .micro_posts_client()
                    .update_micro_post(request)
                    .await
                    .map_err(TonicError::server_returned_status)?;
            }
            Ok(None) => {
                debug!("Self referncing post: ignoring post");
            }
            Err(err) => {
                error!("Error processing post: {:?}", err);
            }
        }
    }

    Ok(())
}
