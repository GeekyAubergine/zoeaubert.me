use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::status_lol_load_data_from_archive_job::StatusLolLoadFromArchiveJob;
use super::jobs::status_lol_save_data_to_archive_job::StatusLolSaveDataToArchiveJob;
use super::jobs::status_lol_download_data_job::StatusLolDownloadDataJob;

pub struct StatusLolListener;

impl StatusLolListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for StatusLolListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(StatusLolLoadFromArchiveJob::new())
                    .await?;
            }
            Event::StatusLolRepoLoadedFromArchive => {
                app_state
                    .dispatch_job(StatusLolDownloadDataJob::new())
                    .await?;
            }
            Event::StatusLolRepoUpdated => {
                app_state
                    .dispatch_job(StatusLolSaveDataToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
