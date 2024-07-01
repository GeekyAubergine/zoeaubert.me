use async_trait::async_trait;
use tracing::{info, warn};

use crate::{
    application::{
        events::Event, jobs::games::fetch_games_data_from_steam_job::GamesDownloadDataJob,
    },
    infrastructure::{
        app_state::AppState, bus::job_runner::Job, services::archive::load_archive_file,
    },
    prelude::Result,
    GAMES_ARCHIVE_FILENAME,
};

#[derive(Debug)]
pub struct LoadGamesFromArchiveJob;
impl LoadGamesFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait]
impl Job for LoadGamesFromArchiveJob {
    fn name(&self) -> &str {
        "LoadGamesFromArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading games archive");
        match load_archive_file(app_state.config(), GAMES_ARCHIVE_FILENAME).await {
            Ok(games_archive) => {
                app_state
                    .games_repo()
                    .rebuild_from_archive(games_archive)
                    .await;

                app_state
                    .dispatch_event(Event::GamesRepoLoadedFromArchive)
                    .await
            }
            Err(err) => {
                warn!("Failed to load games archive: {:?}", err);
                app_state.dispatch_job(GamesDownloadDataJob::new()).await
            }
        }
    }
}
