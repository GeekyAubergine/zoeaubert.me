use crate::application::jobs::games::fetch_games_data_from_steam_job::GamesDownloadDataJob;
use crate::application::jobs::games::load_games_from_archive_job::LoadGamesFromArchiveJob;
use crate::application::jobs::games::save_games_to_archive_job::SaveGamesToArchiveJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

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
                    .dispatch_job(LoadGamesFromArchiveJob::new())
                    .await?;
            }
            Event::GamesRepoLoadedFromArchive => {
                app_state.dispatch_job(GamesDownloadDataJob::new()).await?;
            }
            Event::GamesRepoUpdated => {
                app_state
                    .dispatch_job(SaveGamesToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
