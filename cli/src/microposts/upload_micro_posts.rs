use serde::Deserialize;
use shared::{
    utils::date::parse_date,
    zoeaubert_proto::webserver::{MicroPost, UpdateMicroPostRequest},
};
use tonic::Request;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    error::{Error, TonicError},
    prelude::Result,
    utils::{
        api_client::ApiClient,
        fs::{find_files_rescurse, read_file_from_content_dir},
        image_extraction::extract_images_from_html,
    },
};

use super::error::MicroPostError;

const MICRO_POSTS_DIR: &str = "micros";

const NAMESPACE: Uuid = Uuid::from_u128(0x0d2f1b7e33246452345);

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

    let date = parse_date(front_matter.date.as_str()).map_err(Error::DateParse)?;

    let slug_date = date.format("%Y-%m-%d").to_string();

    let file_name = file_path
        .split('/')
        .last()
        .ok_or(MicroPostError::invalid_file_path(file_path.to_string()))?;

    let file_name = file_name
        .split('.')
        .next()
        .ok_or(MicroPostError::invalid_file_name(file_path.to_string()))?;

    let slug = format!("{}-{}", slug_date, file_name);

    let images = extract_images_from_html(&content, &date);

    Ok(MicroPost {
        uuid: Uuid::new_v5(&NAMESPACE, slug.as_bytes()).into(),
        slug: slug.clone(),
        date: date.to_rfc3339(),
        content: content.to_string(),
        tags: front_matter.tags,
        images,
    })
}

pub async fn upload_micro_posts(api_client: &ApiClient) -> Result<()> {
    info!("Uploading micro posts");
    let files = find_files_rescurse(MICRO_POSTS_DIR, "md")?;

    for file in files {
        let content = match read_file_from_content_dir(&file).await {
            Ok(content) => content,
            Err(err) => {
                error!("Error reading file: {:?}", err);
                continue;
            }
        };

        match file_to_micro_post(&file, &content).await {
            Ok(micro_post) => {
                info!("Uploading post: {}", micro_post.slug);

                let request = Request::new(UpdateMicroPostRequest {
                    micro_post: Some(micro_post),
                });

                api_client
                    .micro_posts_client()
                    .update_micro_post(request)
                    .await
                    .map_err(TonicError::server_returned_status)?;
            }
            Err(err) => {
                error!("Error processing post: {:?}", err);
            }
        }
    }

    Ok(())
}
