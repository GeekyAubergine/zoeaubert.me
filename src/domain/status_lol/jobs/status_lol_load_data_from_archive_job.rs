use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event, domain::status_lol::jobs::status_lol_download_data_job::StatusLolDownloadDataJob, infrastructure::{app_state::AppState, bus::job_runner::Job}, load_archive_file, prelude::Result, save_archive_file, STATUS_LOL_ARCHIVE_FILENAME
};

#[derive(Debug)]
pub struct StatusLolLoadFromArchiveJob;
impl StatusLolLoadFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for StatusLolLoadFromArchiveJob {
    fn name(&self) -> &str {
        "StatusLolLoadFromArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        match load_archive_file(app_state.config(), STATUS_LOL_ARCHIVE_FILENAME).await {
            Ok(status_lol_archive) => {
                app_state
                    .status_lol_repo()
                    .rebuild_from_archive(status_lol_archive)
                    .await;

                app_state
                    .dispatch_event(Event::StatusLolRepoLoadedFromArchive)
                    .await
            }
            Err(err) => {
                warn!("Failed to load status_lol archive: {:?}", err);
                app_state.dispatch_job(StatusLolDownloadDataJob::new()).await
            }
        }
    }
}
