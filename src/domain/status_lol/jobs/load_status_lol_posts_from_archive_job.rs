use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event, domain::status_lol::jobs::fetch_status_lol_posts_job::FetchStatusLolPostsJob, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result, utils::archive::load_archive_file, STATUS_LOL_ARCHIVE_FILENAME
};

#[derive(Debug)]
pub struct LoadStatusLolPostsFromArchiveJob;
impl LoadStatusLolPostsFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadStatusLolPostsFromArchiveJob {
    fn name(&self) -> &str {
        "LoadStatusLolPostsFromArchiveJob"
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
                app_state.dispatch_job(FetchStatusLolPostsJob::new()).await
            }
        }
    }
}
