use async_trait::async_trait;

use crate::{
    application::events::Event, infrastructure::{app_state::AppState, bus::job_runner::Job}, load_archive_file, prelude::Result
};

#[derive(Debug)]
pub struct ReloadBlogPostsJob;

impl ReloadBlogPostsJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadBlogPostsJob {
    fn name(&self) -> &str {
        "ReloadBlogPostsJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .blog_posts_repo()
            .reload(app_state.config(), app_state.content_dir())
            .await?;

        app_state
            .dispatch_event(Event::BlogPostsRepoUpdated)
            .await?;

        Ok(())
    }
}
