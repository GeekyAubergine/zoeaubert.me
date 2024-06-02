use async_trait::async_trait;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    save_archive_file, STATUS_LOL_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct LoadStatusLolDataFromArchiveJob;
impl LoadStatusLolDataFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadStatusLolDataFromArchiveJob {
    fn name(&self) -> &str {
        "LoadStatusLolDataFromArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let data = load_archive_file(app_state.config(), STATUS_LOL_ARCHIVE_FILENAME).await?;

        app_state.status_lol_repo().load_from_archive(data).await;

        app_state
            .dispatch_event(Event::StatusLolRepoLoadedFromArchive)
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadStatusLolDataJob;
impl ReloadStatusLolDataJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for ReloadStatusLolDataJob {
    fn name(&self) -> &str {
        "ReloadStatusLolDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .status_lol_repo()
            .reload(app_state.config())
            .await?;

        app_state
            .dispatch_event(Event::StatusLolRepoUpdated)
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SaveStatusLolDataToArchiveJob;
impl SaveStatusLolDataToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveStatusLolDataToArchiveJob {
    fn name(&self) -> &str {
        "SaveStatusLolDataToArchiveJob"
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
