use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::{mpsc::Sender, RwLock};

use crate::{
    application::events::Event, error::Error, infrastructure::config::Config, prelude::*,
    GAMES_ARCHIVE_FILENAME, LEGO_ARCHIVE_FILENAME, STATUS_LOL_ARCHIVE_FILENAME,
};

use super::{
    bus::job_runner::{Job, JobPriority},
    config::SiteConfig,
    query_services::tags_query_service::TagsQueryService,
    repos::{
        about_repo::AboutRepo, album_photos_repo::AlbumPhotosRepo, albums_repo::AlbumsRepo,
        blog_posts_repo::BlogPostsRepo, faq_repo::FaqRepo,
        game_achievements_repo::GameAchievementsRepo, games_repo::GamesRepo,
        images_repo::ImagesRepo, lego_minifigs_repo::LegoMinifigsRepo,
        lego_sets_repo::LegoSetsRepo, mastodon_posts_repo::MastodonPostsRepo,
        micro_posts_repo::MicroPostsRepo, microblog_archive_repo::MicroblogArchiveRepo,
        silly_names_repo::SillyNamesRepo, status_lol_repo::StatusLolPostsRepo, tags_repo::TagsRepo,
    },
    services::{cache::Cache, cdn::Cdn, content_dir::ContentDir},
};

pub type DatabaseConnection = Pool<Postgres>;

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
    game_achievements_repo: GameAchievementsRepo,
    lego_set_repo: LegoSetsRepo,
    lego_minifigs_repo: LegoMinifigsRepo,
    status_lol_posts_repo: StatusLolPostsRepo,
    about_repo: AboutRepo,
    faq_repo: FaqRepo,
    silly_names_repo: SillyNamesRepo,
    blog_posts_repo: BlogPostsRepo,
    micro_posts_repo: MicroPostsRepo,
    microblog_archive_repo: MicroblogArchiveRepo,
    mastodon_posts_repo: MastodonPostsRepo,
    albums_repo: AlbumsRepo,
    album_photos_repo: AlbumPhotosRepo,
    tags_repo: TagsRepo,
    images_repo: ImagesRepo,
}

impl AppStateData {
    pub async fn new(
        database_connection: DatabaseConnection,
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
            games_repo: GamesRepo::new(database_connection.clone()),
            game_achievements_repo: GameAchievementsRepo::new(database_connection.clone()),
            lego_set_repo: LegoSetsRepo::new(database_connection.clone()),
            lego_minifigs_repo: LegoMinifigsRepo::new(database_connection.clone()),
            status_lol_posts_repo: StatusLolPostsRepo::new(database_connection.clone()),
            about_repo: AboutRepo::default(),
            faq_repo: FaqRepo::default(),
            silly_names_repo: SillyNamesRepo::new(database_connection.clone()),
            blog_posts_repo: BlogPostsRepo::default(),
            micro_posts_repo: MicroPostsRepo::default(),
            microblog_archive_repo: MicroblogArchiveRepo::default(),
            mastodon_posts_repo: MastodonPostsRepo::default(),
            albums_repo: AlbumsRepo::default(),
            album_photos_repo: AlbumPhotosRepo::new(database_connection.clone()),
            tags_repo: TagsRepo::new(database_connection.clone()),
            images_repo: ImagesRepo::new(database_connection.clone()),
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

    pub fn game_achievements_repo(&self) -> &GameAchievementsRepo {
        &self.game_achievements_repo
    }

    pub fn lego_set_repo(&self) -> &LegoSetsRepo {
        &self.lego_set_repo
    }

    pub fn lego_minifigs_repo(&self) -> &LegoMinifigsRepo {
        &self.lego_minifigs_repo
    }

    pub fn status_lol_repo(&self) -> &StatusLolPostsRepo {
        &self.status_lol_posts_repo
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

    pub fn album_photos_repo(&self) -> &AlbumPhotosRepo {
        &self.album_photos_repo
    }

    pub fn tags_repo(&self) -> &TagsRepo {
        &self.tags_repo
    }

    pub fn images_repo(&self) -> &ImagesRepo {
        &self.images_repo
    }
}

pub type AppState = Arc<AppStateData>;
