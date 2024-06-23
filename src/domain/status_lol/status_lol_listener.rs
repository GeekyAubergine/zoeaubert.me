use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::load_status_lol_posts_from_archive_job::LoadStatusLolPostsFromArchiveJob;
use super::jobs::save_status_lol_posts_to_archive_job::SaveStatusLolPostsToArchiveJob;
use super::jobs::fetch_status_lol_posts_job::FetchStatusLolPostsJob;

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
                    .dispatch_job(LoadStatusLolPostsFromArchiveJob::new())
                    .await?;
            }
            Event::StatusLolRepoLoadedFromArchive => {
                app_state
                    .dispatch_job(FetchStatusLolPostsJob::new())
                    .await?;
            }
            Event::StatusLolRepoUpdated => {
                app_state
                    .dispatch_job(SaveStatusLolPostsToArchiveJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
