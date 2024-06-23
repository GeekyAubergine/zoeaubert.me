use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    application::events::Event,
    domain::{micro_posts::micro_posts_models::MicroPost, models::tag::Tag},
    error::Error,
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
    },
    prelude::Result,
    utils::{find_files_rescurse, parse_date},
};

const MICRO_POSTS_DIR: &str = "micros";

#[derive(Debug, Clone, Deserialize)]
pub struct MicroPostFrontMatter {
    date: String,
    tags: Vec<String>,
}

fn front_matter_from_string(s: &str) -> Result<MicroPostFrontMatter> {
    serde_yaml::from_str(s).map_err(Error::ParseMicroPostFrontMatter)
}

fn file_to_micro_post(file_path: String, s: &str) -> Result<MicroPost> {
    let split = s.split("---").collect::<Vec<&str>>();

    let front_matter = split.get(1);
    let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

    let content = s.get(front_matter_len + 6..);

    match (front_matter, content) {
        (Some(front_matter), Some(content)) => {
            let front_matter = front_matter_from_string(front_matter)?;

            let date = parse_date(front_matter.date.as_str())?;

            let tags = front_matter
                .tags
                .iter()
                .map(|tag| Tag::from_string(tag))
                .collect::<Vec<Tag>>();

            let slug_date = date.format("%Y-%m-%d").to_string();

            let file_name = file_path
                .split('/')
                .last()
                .ok_or(Error::ParseMicroPost("Invalid file path".to_string()))?;

            let file_name = file_name
                .split('.')
                .next()
                .ok_or(Error::ParseMicroPost("Invalid file name".to_string()))?;

            let slug = format!("{}-{}", slug_date, file_name);

            Ok(MicroPost::new(slug, date, content.to_string(), tags))
        }
        _ => Err(Error::ParseMicroPost("Invalid front matter".to_string())),
    }
}

#[derive(Debug)]
pub struct LoadMicroPostsJob;

impl LoadMicroPostsJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadMicroPostsJob {
    fn name(&self) -> &str {
        "LoadMicroPostsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let micro_post_files = find_files_rescurse(MICRO_POSTS_DIR, "md", app_state.config())?;

        for file in micro_post_files {
            let file_content = app_state
                .content_dir()
                .read_file(&file, app_state.config())
                .await?;

            let micro_post = file_to_micro_post(file, &file_content)?;

            app_state.micro_posts_repo().commit(micro_post).await;
        }

        app_state.dispatch_event(Event::MicroPostsRepoUpdated).await;

        Ok(())
    }
}
