use crate::application::jobs::faq::load_faq_job::LoadFaqJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::infrastructure::bus::job_runner::JobPriority;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

pub struct FaqListener;

impl FaqListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for FaqListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        if let Event::ServerBooted = event {
            app_state
                .dispatch_job(LoadFaqJob::new(), JobPriority::High)
                .await?;
        }

        Ok(())
    }
}
