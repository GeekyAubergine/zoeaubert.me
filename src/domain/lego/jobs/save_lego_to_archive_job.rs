use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    prelude::Result,
    utils::archive::save_archive_file,
    LEGO_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct SaveLegoToArchiveJob;
impl SaveLegoToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveLegoToArchiveJob {
    fn name(&self) -> &str {
        "SaveLegoToArchiveJob"
    }
    async fn run(&self, app_state: &AppState) -> Result<()> {
        let lego = app_state.lego_repo().get_archived().await;

        save_archive_file(app_state.config(), &lego, LEGO_ARCHIVE_FILENAME).await?;

        app_state.dispatch_event(Event::LegoRepoArchived).await?;

        Ok(())
    }
}
