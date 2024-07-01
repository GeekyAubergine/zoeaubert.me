use async_trait::async_trait;
use tracing::info;

use crate::{
    application::events::Event, domain::models::faq::Faq, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result
};

const FILE_NAME: &str = "faq.md";

#[derive(Debug)]
pub struct LoadFaqJob;
impl LoadFaqJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadFaqJob {
    fn name(&self) -> &str {
        "LoadFaqJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading faq data");
        let faq_content = app_state
            .content_dir()
            .read_file(FILE_NAME, app_state.config())
            .await?;

        let faq = Faq::new(faq_content);

        app_state.faq_repo().commit(faq);

        app_state.dispatch_event(Event::FaqRepoUpdated).await?;

        Ok(())
    }
}
