use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    save_archive_file, LEGO_ARCHIVE_FILENAME,
};


#[derive(Debug)]
pub struct SaveLegoDataToArchiveJob;
impl SaveLegoDataToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveLegoDataToArchiveJob {
    fn name(&self) -> &str {
        "SaveLegoDataToArchiveJob"
    }
    async fn run(&self, app_state: &AppState) -> Result<()> {
        let lego = app_state.lego_repo().get_archived().await;

        save_archive_file(app_state.config(), &lego, LEGO_ARCHIVE_FILENAME).await?;

        app_state.dispatch_event(Event::LegoRepoArchived).await?;

        Ok(())
    }
}
