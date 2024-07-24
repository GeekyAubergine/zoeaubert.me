use std::sync::Arc;

use tokio::sync::{mpsc::Sender, RwLock};

use crate::{
    application::events::Event, error::Error, infrastructure::config::Config, prelude::*,
    GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use super::{
    bus::job_runner::{Job, JobPriority},
    config::SiteConfig,
    repos::{
        about_repo::AboutRepo, albums_repo::AlbumsRepo, blog_posts_repo::BlogPostsRepo,
        faq_repo::FaqRepo, games_repo::GamesRepo, lego_repo::LegoRepo,
        mastodon_posts_repo::MastodonPostsRepo, micro_posts_repo::MicroPostsRepo,
        microblog_archive_repo::MicroblogArchiveRepo, silly_names_repo::SillyNamesRepo,
        status_lol_repo::StatusLolRepo,
    },
    services::{cache::Cache, cdn::Cdn, content_dir::ContentDir},
};

#[derive(Debug, Clone)]
pub struct AppStateData {
    event_sender: Sender<Event>,
    job_high_priority_sender: Sender<Box<dyn Job>>,
    job_normal_priority_sender: Sender<Box<dyn Job>>,
    job_low_priority_sender: Sender<Box<dyn Job>>,
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
    microblog_archive_repo: MicroblogArchiveRepo,
    mastodon_posts_repo: MastodonPostsRepo,
    albums_repo: AlbumsRepo,
}

impl AppStateData {
    pub async fn new(
        config: &Config,
        event_sender: Sender<Event>,
        job_high_priority_sender: Sender<Box<dyn Job>>,
        job_normal_priority_sender: Sender<Box<dyn Job>>,
        job_low_priority_sender: Sender<Box<dyn Job>>,
    ) -> Self {
        Self {
            event_sender,
            job_high_priority_sender,
            job_normal_priority_sender,
            job_low_priority_sender,
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
            microblog_archive_repo: MicroblogArchiveRepo::default(),
            mastodon_posts_repo: MastodonPostsRepo::default(),
            albums_repo: AlbumsRepo::default(),
        }
    }

    pub async fn dispatch_job<J: Job + 'static>(
        &self,
        job: J,
        priority: JobPriority,
    ) -> Result<()> {
        let job = Box::new(job);
        match priority {
            JobPriority::High => self
                .job_high_priority_sender
                .send(job)
                .await
                .map_err(Error::DispatchJob),
            JobPriority::Normal => self
                .job_normal_priority_sender
                .send(job)
                .await
                .map_err(Error::DispatchJob),
            JobPriority::Low => self
                .job_low_priority_sender
                .send(job)
                .await
                .map_err(Error::DispatchJob),
        }
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

    pub fn microblog_archive_repo(&self) -> &MicroblogArchiveRepo {
        &self.microblog_archive_repo
    }

    pub fn mastodon_posts_repo(&self) -> &MastodonPostsRepo {
        &self.mastodon_posts_repo
    }

    pub fn albums_repo(&self) -> &AlbumsRepo {
        &self.albums_repo
    }
}

pub type AppState = Arc<AppStateData>;
