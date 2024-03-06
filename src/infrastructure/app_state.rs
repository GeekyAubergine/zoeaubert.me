use std::sync::Arc;

use tokio::sync::{mpsc::Sender, RwLock};

use crate::{
    application::events::Event,
    error::Error,
    infrastructure::{
        config::Config,
        repositories::{
            games_repo::GamesRepo, lego_repo::LegoRepo, status_lol_repo::StatusLolRepo,
        },
    },
    load_archive_file,
    prelude::*,
    GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use super::{
    bus::job_runner::Job, cache::Cache, cdn::Cdn, repositories::{about_repo::AboutRepo, faq_repo::FaqRepo}
};

#[derive(Debug, Clone)]
pub struct AppStateData {
    config: Config,
    cdn: Cdn,
    cache: Cache,
    games_repo: GamesRepo,
    lego_repo: LegoRepo,
    status_lol_repo: StatusLolRepo,
    about_repo: AboutRepo,
    faq_repo: FaqRepo,
    job_sender: Sender<Box<dyn Job>>,
    event_sender: Sender<Event>,
}

impl AppStateData {
    pub async fn new(
        config: &Config,
        job_sender: Sender<Box<dyn Job>>,
        event_sender: Sender<Event>,
    ) -> Self {
        Self {
            config: config.clone(),
            cdn: Cdn::new(config).await,
            cache: Cache::new(),
            games_repo: GamesRepo::new(),
            lego_repo: LegoRepo::new(),
            status_lol_repo: StatusLolRepo::new(),
            about_repo: AboutRepo::new(),
            faq_repo: FaqRepo::new(),
            job_sender,
            event_sender,
        }
    }

    pub async fn load_from_archive(&mut self) -> Result<()> {
        match load_archive_file(self.config(), GAMES_ARCHIVE_FILENAME).await {
            Ok(games_archive) => self.games_repo = GamesRepo::from_archive(games_archive),
            Err(_) => {}
        }

        match load_archive_file(self.config(), LEGO_ARCHIVE_FILENAME).await {
            Ok(lego_archive) => self.lego_repo = LegoRepo::from_archive(lego_archive),
            Err(_) => {}
        }

        match load_archive_file(self.config(), STATUS_LOL_ARCHIVE_FILENAME).await {
            Ok(status_lol_archive) => {
                self.status_lol_repo = StatusLolRepo::from_archive(status_lol_archive)
            }
            Err(_) => {}
        }

        Ok(())
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

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn cdn(&self) -> &Cdn {
        &self.cdn
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
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
}

pub type AppState = Arc<AppStateData>;
