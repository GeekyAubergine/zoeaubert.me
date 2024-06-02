use async_trait::async_trait;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    save_archive_file, GAMES_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct LoadGamesDataFromArchiveJob;
impl LoadGamesDataFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadGamesDataFromArchiveJob {
    fn name(&self) -> &str {
        "LoadGamesDataFromArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let games_archive = load_archive_file(app_state.config(), GAMES_ARCHIVE_FILENAME).await?;

        app_state
            .games_repo()
            .load_from_archive(games_archive)
            .await;

        app_state
            .dispatch_event(Event::GamesRepoLoadedFromArchive)
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadGamesDataJob;
impl ReloadGamesDataJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for ReloadGamesDataJob {
    fn name(&self) -> &str {
        "ReloadGamesDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.games_repo().reload(app_state.config()).await?;

        app_state.dispatch_event(Event::GamesRepoUpdated).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SaveGamesDataToArchiveJob;
impl SaveGamesDataToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveGamesDataToArchiveJob {
    fn name(&self) -> &str {
        "SaveGamesDataToArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let games = app_state.games_repo().get_archived().await;

        save_archive_file(app_state.config(), &games, GAMES_ARCHIVE_FILENAME).await?;

        app_state.dispatch_event(Event::GamesRepoArchived).await?;

        Ok(())
    }
}
