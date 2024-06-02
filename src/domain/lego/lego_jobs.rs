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
pub struct LoadLegoDataFromArchiveJob;
impl LoadLegoDataFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadLegoDataFromArchiveJob {
    fn name(&self) -> &str {
        "LoadLegoDataFromArchiveJob"
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
                app_state.dispatch_job(RealoadLegoDataJob::new()).await
            }
        }
    }
}

#[derive(Debug)]
pub struct RealoadLegoDataJob;
impl RealoadLegoDataJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for RealoadLegoDataJob {
    fn name(&self) -> &str {
        "RealoadLegoDataJob"
    }
    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.lego_repo().reload(app_state.config()).await?;

        app_state.dispatch_event(Event::LegoRepoUpdated).await?;

        Ok(())
    }
}

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
