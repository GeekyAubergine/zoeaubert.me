use async_trait::async_trait;
use tracing::warn;

use crate::{
    application::events::Event, infrastructure::{app_state::AppState, bus::job_runner::Job}, prelude::Result, utils::archive::save_archive_file, GAMES_ARCHIVE_FILENAME, MASTODON_ARCHIVE_FILENAME
};

#[derive(Debug)]
pub struct SaveMastodonPostsToArchiveJob;
impl SaveMastodonPostsToArchiveJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for SaveMastodonPostsToArchiveJob {
    fn name(&self) -> &str {
        "SaveMastodonPostsToArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        let mastodon_posts = app_state.mastodon_posts_repo().get_archived().await;

        save_archive_file(app_state.config(), &mastodon_posts, MASTODON_ARCHIVE_FILENAME).await?;

        app_state.dispatch_event(Event::MastodonPostsRepoArchived).await?;

        Ok(())
    }
}