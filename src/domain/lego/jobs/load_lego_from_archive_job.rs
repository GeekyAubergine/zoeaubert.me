use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event, domain::lego::jobs::fetch_lego_job::FetchLegoJob, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result, utils::archive::load_archive_file, LEGO_ARCHIVE_FILENAME
};

#[derive(Debug)]
pub struct LoadLegoFromArchiveJob;
impl LoadLegoFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadLegoFromArchiveJob {
    fn name(&self) -> &str {
        "LoadLegoFromArchiveJob"
    }
    async fn run(&self, app_state: &AppState) -> Result<()> {
        match load_archive_file(app_state.config(), LEGO_ARCHIVE_FILENAME).await {
            Ok(lego_archive) => {
                app_state.lego_repo().load_from_archive(lego_archive).await;

                app_state
                    .dispatch_event(Event::LegoRepoLoadedFromArchive)
                    .await
            }
            Err(err) => {
                warn!("Failed to load lego archive: {:?}", err);
                app_state.dispatch_job(FetchLegoJob::new()).await
            }
        }
    }
}
