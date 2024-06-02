use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::games_jobs::{LoadGamesDataFromArchiveJob, ReloadGamesDataJob, SaveGamesDataToArchiveJob};

pub struct GamesListener;

impl GamesListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for GamesListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(LoadGamesDataFromArchiveJob::new())
                    .await?;
            }
            Event::GamesRepoLoadedFromArchive => {
                app_state.dispatch_job(ReloadGamesDataJob::new()).await?;
            }
            Event::GamesRepoUpdated => {
                app_state
                    .dispatch_job(SaveGamesDataToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
