use async_trait::async_trait;

use crate::{
    infrastructure::{app_state::AppState, bus::job_runner::Job},
    load_archive_file,
    prelude::Result,
    GAMES_ARCHIVE_FILENAME,
};

use super::events::Event;

#[derive(Debug)]
pub struct ReloadAllDataJob;

impl ReloadAllDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadAllDataJob {
    fn name(&self) -> &str {
        "ReloadAllDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.dispatch_job(RealoadLegoDataJob::new()).await?;
        app_state.dispatch_job(ReloadGamesDataJob::new()).await?;
        app_state
            .dispatch_job(ReloadStatusLolDataJob::new())
            .await?;
        app_state.dispatch_job(ReloadAboutDataJob::new()).await?;
        app_state.dispatch_job(ReloadFaqDataJob::new()).await?;
        app_state
            .dispatch_job(ReloadSillyNamesDataJob::new())
            .await?;
        app_state.dispatch_job(ReloadBlogPostsJob::new()).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct RealoadLegoDataJob;

impl RealoadLegoDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for RealoadLegoDataJob {
    fn name(&self) -> &str {
        "RealoadLegoDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.lego_repo().reload(app_state.config()).await?;

        app_state.dispatch_event(Event::lego_repo_updated()).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadGamesDataJob;

impl ReloadGamesDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadGamesDataJob {
    fn name(&self) -> &str {
        "ReloadGamesDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.games_repo().reload(app_state.config()).await?;

        app_state
            .dispatch_event(Event::games_repo_updated())
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadStatusLolDataJob;

impl ReloadStatusLolDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadStatusLolDataJob {
    fn name(&self) -> &str {
        "ReloadStatusLolDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .status_lol_repo()
            .reload(app_state.config())
            .await?;

        app_state
            .dispatch_event(Event::status_lol_repo_updated())
            .await?;

        Ok(())
    }
}

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
            .dispatch_event(Event::about_repo_updated())
            .await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadFaqDataJob;

impl ReloadFaqDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadFaqDataJob {
    fn name(&self) -> &str {
        "ReloadFaqDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .faq_repo()
            .reload(app_state.config(), app_state.content_dir())
            .await?;

        app_state.dispatch_event(Event::faq_repo_updated()).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ReloadSillyNamesDataJob;

impl ReloadSillyNamesDataJob {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Job for ReloadSillyNamesDataJob {
    fn name(&self) -> &str {
        "ReloadSillyNamesDataJob"
    }

    async fn run(&self, app_state: &AppState) -> Result<()> {
        app_state
            .silly_names_repo()
            .reload(app_state.config(), app_state.content_dir())
            .await?;

        app_state
            .dispatch_event(Event::silly_names_repo_updated())
            .await?;

        Ok(())
    }
}

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
            .dispatch_event(Event::blog_posts_repo_updated())
            .await?;

        Ok(())
    }
}
