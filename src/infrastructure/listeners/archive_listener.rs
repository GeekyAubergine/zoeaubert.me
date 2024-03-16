use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME,
    STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

pub struct ArchiveListener;

impl ArchiveListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for ArchiveListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::GamesRepoUpdated => {
                let games = app_state.games_repo().get_archived().await;

                save_archive_file(app_state.config(), &games, GAMES_ARCHIVE_FILENAME).await?;

                app_state
                    .dispatch_event(Event::games_repo_archived())
                    .await?;
            }
            Event::LegoRepoUpdated => {
                let lego = app_state.lego_repo().get_archived().await;

                save_archive_file(app_state.config(), &lego, LEGO_ARCHIVE_FILENAME).await?;

                app_state
                    .dispatch_event(Event::lego_repo_archived())
                    .await?;
            }
            Event::StatusLolRepoUpdated => {
                let status_lol = app_state.status_lol_repo().get_archived().await;

                save_archive_file(app_state.config(), &status_lol, STATUS_LOL_ARCHIVE_FILENAME)
                    .await?;

                app_state
                    .dispatch_event(Event::status_lol_repo_archived())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
