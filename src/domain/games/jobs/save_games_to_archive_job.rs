use async_trait::async_trait;
use tracing::{info, warn};

use crate::{
    application::events::Event, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result, utils::archive::save_archive_file, GAMES_ARCHIVE_FILENAME
};

#[derive(Debug)]
pub struct SaveGamesToArchiveJob;
impl SaveGamesToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for SaveGamesToArchiveJob {
    fn name(&self) -> &str {
        "SaveGamesToArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Saving games archive");
        let games = app_state.games_repo().get_archived().await;

        save_archive_file(app_state.config(), &games, GAMES_ARCHIVE_FILENAME).await?;

        app_state.dispatch_event(Event::GamesRepoArchived).await?;

        Ok(())
    }
}
