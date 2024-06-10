use async_trait::async_trait;

use crate::{
    application::events::Event, domain::about::about_models::About, infrastructure::{app_state::AppState, bus::job_runner::Job}, load_archive_file, prelude::Result, save_archive_file, GAMES_ARCHIVE_FILENAME
};

const FILE_NAME_SHORT: &str = "about_short.md";
const FILE_NAME_LONG: &str = "about_long.md";

#[derive(Debug)]
pub struct LoadAboutDataJob;

impl LoadAboutDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadAboutDataJob {
    fn name(&self) -> &str {
        "LoadAboutDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let short = app_state
            .content_dir()
            .read_file(FILE_NAME_SHORT, app_state.config())
            .await?;

        let long = app_state
            .content_dir()
            .read_file(FILE_NAME_LONG, app_state.config())
            .await?;

        let about = About::new(short, long);

        app_state.about_repo().commit(about).await;

        app_state.dispatch_event(Event::AboutRepoUpdated).await?;

        Ok(())
    }
}
