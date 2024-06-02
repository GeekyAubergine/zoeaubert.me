use async_trait::async_trait;

use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    save_archive_file, GAMES_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct ReloadAboutDataJob;

impl ReloadAboutDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadAboutDataJob {
    fn name(&self) -> &str {
        "ReloadAboutDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .about_repo()
            .reload(app_state.config(), app_state.content_dir())
            .await?;

        app_state
            .dispatch_event(Event::AboutRepoUpdated)
            .await?;

        Ok(())
    }
}
