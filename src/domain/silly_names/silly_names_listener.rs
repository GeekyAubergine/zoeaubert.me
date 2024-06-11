use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::load_silly_names_job::LoadSillyNamesJob;

pub struct SillyNamesListener;

impl SillyNamesListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for SillyNamesListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        if let Event::ServerBooted = event {
            app_state.dispatch_job(LoadSillyNamesJob::new()).await?;
        }

        Ok(())
    }
}
