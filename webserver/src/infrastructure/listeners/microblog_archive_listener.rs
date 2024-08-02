use async_trait::async_trait;

use crate::{
    application::{events::Event, jobs::microblog_archive::load_microblog_archive_job::LoadMicroblogArchiveJob},
    infrastructure::{app_state::AppState, bus::{event_queue::EventListener, job_runner::JobPriority}},
    prelude::Result,
};

pub struct MicroblogArchiveListener;

impl MicroblogArchiveListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for MicroblogArchiveListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        if let Event::ServerBooted = event {
            app_state
                .dispatch_job(LoadMicroblogArchiveJob::new(), JobPriority::High)
                .await?;
        }

        Ok(())
    }
}
