use crate::infrastructure::bus::event_queue::EventListener;
use crate::{
    load_archive_file, prelude::*, save_archive_file, GAMES_ARCHIVE_FILENAME,
    LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use async_trait::async_trait;

use crate::application::events::Event;

use crate::infrastructure::app_state::AppState;

use super::blog_posts_jobs::ReloadBlogPostsJob;

pub struct BlogPostsListener;

impl BlogPostsListener {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventListener for BlogPostsListener {
    async fn on_event(&self, event: &Event, app_state: &AppState) -> Result<()> {
        match event {
            Event::ServerBooted => {
                app_state
                    .dispatch_job(ReloadBlogPostsJob::new())
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
