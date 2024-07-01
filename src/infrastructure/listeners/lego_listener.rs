use crate::application::jobs::lego::load_lego_from_archive_job::LoadLegoFromArchiveJob;
use crate::application::jobs::lego::save_lego_to_archive_job::SaveLegoToArchiveJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

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
