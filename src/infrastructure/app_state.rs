use std::sync::Arc;

use tokio::sync::{mpsc::Sender, RwLock};

use crate::{
    application::events::Event,
    domain::{about::about_repo::AboutRepo, blog_posts::blog_posts_repo::BlogPostsRepo, faq::faq_repo::FaqRepo, games::games_repo::GamesRepo, lego::lego_repo::LegoRepo, micro_posts::micro_posts_repo::MicroPostsRepo, silly_names::silly_names_repo::SillyNamesRepo, status_lol::status_lol_repo::StatusLolRepo},
    error::Error,
    infrastructure::config::Config,
    load_archive_file,
    prelude::*,
    GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use super::{
    bus::job_runner::Job,
    cache::Cache,
    cdn::Cdn,
    config::SiteConfig,
    content_dir::ContentDir,
};

#[derive(Debug, Clone)]
pub struct AppStateData {
    job_sender: Sender<Box<dyn Job>>,
    event_sender: Sender<Event>,
    config: Config,
    cdn: Cdn,
    cache: Cache,
    content_dir: ContentDir,
    games_repo: GamesRepo,
    lego_repo: LegoRepo,
    status_lol_repo: StatusLolRepo,
    about_repo: AboutRepo,
    faq_repo: FaqRepo,
    silly_names_repo: SillyNamesRepo,
    blog_posts_repo: BlogPostsRepo,
    micro_posts_repo: MicroPostsRepo,
}

impl AppStateData {
    pub async fn new(
        config: &Config,
        job_sender: Sender<Box<dyn Job>>,
        event_sender: Sender<Event>,
    ) -> Self {
        Self {
            job_sender,
            event_sender,
            config: config.clone(),
            cdn: Cdn::new(config).await,
            cache: Cache::default(),
            content_dir: ContentDir::default(),
            games_repo: GamesRepo::default(),
            lego_repo: LegoRepo::default(),
            status_lol_repo: StatusLolRepo::default(),
            about_repo: AboutRepo::default(),
            faq_repo: FaqRepo::default(),
            silly_names_repo: SillyNamesRepo::default(),
            blog_posts_repo: BlogPostsRepo::default(),
            micro_posts_repo: MicroPostsRepo::default(),
        }
    }

    pub async fn dispatch_job<J: Job + 'static>(&self, job: J) -> Result<()> {
        self.job_sender
            .send(Box::new(job))
            .await
            .map_err(Error::DispatchJob)
    }

    pub async fn dispatch_event(&self, event: Event) -> Result<()> {
        self.event_sender
            .send(event)
            .await
            .map_err(Error::DispatchEvent)
    }

    pub fn site(&self) -> &SiteConfig {
        self.config.site()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn cdn(&self) -> &Cdn {
        &self.cdn
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn content_dir(&self) -> &ContentDir {
        &self.content_dir
    }

    pub fn games_repo(&self) -> &GamesRepo {
        &self.games_repo
    }

    pub fn lego_repo(&self) -> &LegoRepo {
        &self.lego_repo
    }

    pub fn status_lol_repo(&self) -> &StatusLolRepo {
        &self.status_lol_repo
    }

    pub fn about_repo(&self) -> &AboutRepo {
        &self.about_repo
    }

    pub fn faq_repo(&self) -> &FaqRepo {
        &self.faq_repo
    }

    pub fn silly_names_repo(&self) -> &SillyNamesRepo {
        &self.silly_names_repo
    }

    pub fn blog_posts_repo(&self) -> &BlogPostsRepo {
        &self.blog_posts_repo
    }

    pub fn micro_posts_repo(&self) -> &MicroPostsRepo {
        &self.micro_posts_repo
    }
}

pub type AppState = Arc<AppStateData>;
