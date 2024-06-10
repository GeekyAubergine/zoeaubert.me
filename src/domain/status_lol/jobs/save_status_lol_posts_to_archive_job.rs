use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    save_archive_file, STATUS_LOL_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct SaveStatusLolPostsToArchiveJob;
impl SaveStatusLolPostsToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveStatusLolPostsToArchiveJob {
    fn name(&self) -> &str {
        "SaveStatusLolPostsToArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let data = app_state.status_lol_repo().get_archived().await;

        save_archive_file(app_state.config(), &data, STATUS_LOL_ARCHIVE_FILENAME).await?;

        app_state
            .dispatch_event(Event::StatusLolRepoArchived)
            .await?;

        Ok(())
    }
}
