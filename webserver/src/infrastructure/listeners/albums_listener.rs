use crate::application::jobs::albums::load_albums_job::LoadAlbumsJob;
use crate::infrastructure::bus::event_queue::EventListener;
use crate::infrastructure::bus::job_runner::JobPriority;
use crate::prelude::Result;

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

pub struct AlbumsListener;

impl AlbumsListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for AlbumsListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(LoadAlbumsJob::new(), JobPriority::High)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
