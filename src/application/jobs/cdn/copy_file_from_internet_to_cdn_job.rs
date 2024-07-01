use async_trait::async_trait;
use tracing::{debug, info};

use crate::{
    infrastructure::{
        app_state::{self, AppState},
        bus::job_runner::Job,
        services::{cache::CachePath, cdn::CdnPath},
    },
    prelude::Result,
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

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let cache_path: CachePath = CachePath::from_url(app_state.config(), &self.url);

        if app_state
            .cdn()
            .file_exists(&self.cdn_path, app_state.config())
            .await?
        {
            debug!("File already exists in CDN [{}], skipping", self.cdn_path);
            return Ok(());
        }

        info!(
            "Copying file from internet [{}] to CDN [{}]",
            self.url, self.cdn_path
        );

        let data = app_state
            .cache()
            .get_file_from_cache_or_download(app_state, &cache_path, &self.url)
            .await?;

        app_state
            .cdn()
            .upload_file_from_bytes(data, &self.cdn_path, app_state.config())
            .await?;

        Ok(())
    }
}
