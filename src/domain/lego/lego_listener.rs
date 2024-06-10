use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::lego_load_data_from_archive_job::LoadLegoDataFromArchiveJob;
use super::jobs::lego_save_data_to_archive_job::SaveLegoDataToArchiveJob;

pub struct LegoListener;

impl LegoListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for LegoListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(LoadLegoDataFromArchiveJob::new())
                    .await?;
            }
            Event::LegoRepoUpdated => {
                app_state
                    .dispatch_job(SaveLegoDataToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
