use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::load_lego_from_archive_job::LoadLegoFromArchiveJob;
use super::jobs::save_lego_to_archive_job::SaveLegoToArchiveJob;

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
                    .dispatch_job(LoadLegoFromArchiveJob::new())
                    .await?;
            }
            Event::LegoRepoUpdated => {
                app_state
                    .dispatch_job(SaveLegoToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
