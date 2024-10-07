use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, info};

use crate::{
    error::Error, infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
        services::{cdn::CdnPath},
    }, prelude::Result
};

#[derive(Debug)]
pub struct CopyFileFromInternetToCdnJob {
    url: String,
    cdn_path: CdnPath,
}

impl CopyFileFromInternetToCdnJob {
    pub fn new(url: String, cdn_path: CdnPath) -> Self {
        Self { url, cdn_path }
    }
}

#[async_trait]
impl Job for CopyFileFromInternetToCdnJob {
    fn name(&self) -> &str {
        "CopyFileFromInternetToCdnJob"
    }

    async fn run(&self, state: &AppState) -> Result<()> {
        if state
            .cdn()
            .file_exists(&self.cdn_path, state.config())
            .await?
        {
            debug!("File already exists in CDN [{}], skipping", self.cdn_path);
            return Ok(());
        }

        info!(
            "Copying file from internet [{}] to CDN [{}]",
            self.url, self.cdn_path
        );

        let reqwest = Client::new()
            .get(self.url.clone())
            .send()
            .await
            .map_err(Error::UrlDownload)?;

        let content = reqwest.bytes().await.map_err(Error::UrlDownload)?.to_vec();

        state
            .cdn()
            .upload_file_from_bytes(content, &self.cdn_path, state.config())
            .await?;

        Ok(())
    }
}
