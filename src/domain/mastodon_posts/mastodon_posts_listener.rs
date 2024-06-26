use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    prelude::*, GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::jobs::fetch_mastodon_posts_job::FetchMastodonPostsJob;
use super::jobs::load_mastodon_posts_from_archive::LoadMastodonPostsFromArchiveJob;
use super::jobs::save_mastodon_posts_to_archive::SaveMastodonPostsToArchiveJob;
pub struct MastodonListener;

impl MastodonListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for MastodonListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state.dispatch_job(LoadMastodonPostsFromArchiveJob::new()).await?;
            }
            Event::MastodonPostsRepoLoadedFromArchive => {
                app_state.dispatch_job(FetchMastodonPostsJob::new()).await?;
            }
            Event::MastodonPostsRepoUpdated => {
                app_state.dispatch_job(SaveMastodonPostsToArchiveJob::new()).await?;
            }
            _ => {}
        }

        Ok(())
    }
}
