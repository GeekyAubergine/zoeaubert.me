use crate::application::jobs::status_lol::fetch_status_lol_posts_job::FetchStatusLolPostsJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::infrastructure::bus::job_runner::JobPriority;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

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
                    .dispatch_job(FetchStatusLolPostsJob::new(), JobPriority::Normal)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
