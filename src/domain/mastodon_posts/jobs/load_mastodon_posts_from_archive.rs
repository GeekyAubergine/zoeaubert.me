use async_trait::async_trait;
use tracing::{info, warn};

use crate::domain::mastodon_posts::jobs::fetch_mastodon_posts_job::FetchMastodonPostsJob;
use crate::MASTODON_ARCHIVE_FILENAME;
use crate::{
    application::events::Event,
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    prelude::Result,
    utils::archive::load_archive_file,
};

#[derive(Debug)]
pub struct LoadMastodonPostsFromArchiveJob;
impl LoadMastodonPostsFromArchiveJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for LoadMastodonPostsFromArchiveJob {
    fn name(&self) -> &str {
        "LoadMastodonPostsFromArchiveJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        info!("Loading mastodon archive");
        match load_archive_file(app_state.config(), MASTODON_ARCHIVE_FILENAME).await {
            Ok(mastodon_archive) => {
                app_state
                    .mastodon_posts_repo()
                    .rebuild_from_archive(mastodon_archive)
                    .await;

                app_state
                    .dispatch_event(Event::MastodonPostsRepoLoadedFromArchive)
                    .await
            }
            Err(err) => {
                warn!("Failed to load games archive: {:?}", err);
                app_state.dispatch_job(FetchMastodonPostsJob::new()).await
            }
        }
    }
}
