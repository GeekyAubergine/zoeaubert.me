use std::sync::Arc;

use super::{
    repositories::{
        about_text_repo_memory::AboutTextRepoMemory, albums_repo_disk::AlbumsRepoDisk, blog_posts_repo_disk::BlogPostsRepoDisk, faq_repo_memory::FaqRepoMemory, leauge_repo_disk::LeagueRepoDisk, lego_repo_disk::LegoRepoDisk, mastodon_post_repo_disk::MastodonPostRepoDisk, micro_blog_repo_disk::MicroPostsRepoDisk, movie_reviews_repo_memory::MovieReviewsRepoMemory, now_text_repo_memory::NowTextRepoMemory, omni_post_repo_memory::OmniPostRepoMemory, profiler_memory::ProfilerMemory, project_repo_disk::ProjectsRepoDisk, referral_repo_memory::ReferralsRepoMemory, silly_names_repo_memory::SillyNamesRepoMemory, steam_achievements_repo_disk::GameAchievementsRepoDisk, steam_games_repo_disk::GamesRepoDisk, tv_show_reviews_repo_memory::TvShowReviewsRepoMemory
    },
    services::{
        book_service_open_library::BookServiceOpenLibrary, cache_service_disk::CacheServiceDisk,
        cdn_service_bunny::CdnServiceBunny, file_service_disk::FileServiceDisk,
        image_service_impl::ImageServiceImpl, movie_service_tmdb::MovieServiceTmdb,
        network_service_reqwest::NetworkServiceReqwest,
        page_rendering_service_impl::PageRenderingServiceImpl,
        query_limiting_service_disk::QueryLimitingServiceDisk,
        tv_shows_service_tmdb::TvShowsServiceTmdb,
    },
};

use crate::{
    domain::{
        repositories::{
            AboutTextRepo, AlbumsRepo, BlogPostsRepo, FaqRepo, LeagueRepo, LegoRepo, MastodonPostsRepo, MicroPostsRepo, MovieReviewsRepo, NowTextRepo, OmniPostRepo, Profiler, ProjectsRepo, ReferralsRepo, SillyNamesRepo, SteamAchievementsRepo, SteamGamesRepo, TvShowReviewsRepo
        },
        services::{
            BookService, CacheService, CdnService, FileService, ImageService, MovieService,
            NetworkService, PageRenderingService, QueryLimitingService, TvShowsService,
        },
        state::State,
    },
    prelude::*,
};

pub struct AppState {
    profiler: ProfilerMemory,
    silly_names_repo: SillyNamesRepoMemory,
    about_text_repo: AboutTextRepoMemory,
    blog_posts_repo: BlogPostsRepoDisk,
    micro_posts_repo: MicroPostsRepoDisk,
    mastodon_posts_repo: MastodonPostRepoDisk,
    lego_repo: LegoRepoDisk,
    games_repo: GamesRepoDisk,
    game_achievements_repo: GameAchievementsRepoDisk,
    movie_reviews_repo: MovieReviewsRepoMemory,
    tv_show_reviews_repo: TvShowReviewsRepoMemory,
    albums_repo: AlbumsRepoDisk,
    referrals_repo: ReferralsRepoMemory,
    faq_repo: FaqRepoMemory,
    now_text_repo: NowTextRepoMemory,
    league_repo: LeagueRepoDisk,
    omni_post_repo: OmniPostRepoMemory,
    projects_repo: ProjectsRepoDisk,
    // services
    cache_service: CacheServiceDisk,
    cdn_service: CdnServiceBunny,
    movie_service: MovieServiceTmdb,
    image_service: ImageServiceImpl,
    network_service: NetworkServiceReqwest,
    file_service: FileServiceDisk,
    query_limiting_service: QueryLimitingServiceDisk,
    tv_shows_service: TvShowsServiceTmdb,
    page_rendering_service: PageRenderingServiceImpl,
    book_service: BookServiceOpenLibrary,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            profiler: ProfilerMemory::new(),
            silly_names_repo: SillyNamesRepoMemory::new(),
            about_text_repo: AboutTextRepoMemory::new(),
            blog_posts_repo: BlogPostsRepoDisk::new().await?,
            micro_posts_repo: MicroPostsRepoDisk::new().await?,
            mastodon_posts_repo: MastodonPostRepoDisk::new().await?,
            lego_repo: LegoRepoDisk::new().await?,
            games_repo: GamesRepoDisk::new().await?,
            game_achievements_repo: GameAchievementsRepoDisk::new().await?,
            movie_reviews_repo: MovieReviewsRepoMemory::new(),
            tv_show_reviews_repo: TvShowReviewsRepoMemory::new(),
            albums_repo: AlbumsRepoDisk::new().await?,
            referrals_repo: ReferralsRepoMemory::new(),
            faq_repo: FaqRepoMemory::new(),
            now_text_repo: NowTextRepoMemory::new(),
            league_repo: LeagueRepoDisk::new().await?,
            omni_post_repo: OmniPostRepoMemory::new(),
            projects_repo: ProjectsRepoDisk::new().await?,
            // services
            cache_service: CacheServiceDisk::new(),
            cdn_service: CdnServiceBunny::new(),
            movie_service: MovieServiceTmdb::new().await?,
            image_service: ImageServiceImpl::new(),
            network_service: NetworkServiceReqwest::new(),
            file_service: FileServiceDisk::new(),
            query_limiting_service: QueryLimitingServiceDisk::new().await?,
            tv_shows_service: TvShowsServiceTmdb::new().await?,
            page_rendering_service: PageRenderingServiceImpl::new(),
            book_service: BookServiceOpenLibrary::new().await?,
        })
    }

    pub fn profiler(&self) -> &impl Profiler {
        &self.profiler
    }
}

impl State for AppState {
    fn profiler(&self) -> &impl Profiler {
        &self.profiler
    }

    fn silly_names_repo(&self) -> &impl SillyNamesRepo {
        &self.silly_names_repo
    }

    fn about_text_repo(&self) -> &impl AboutTextRepo {
        &self.about_text_repo
    }

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo {
        &self.blog_posts_repo
    }

    fn micro_posts_repo(&self) -> &impl MicroPostsRepo {
        &self.micro_posts_repo
    }

    fn mastodon_posts_repo(&self) -> &impl MastodonPostsRepo {
        &self.mastodon_posts_repo
    }

    fn lego_repo(&self) -> &impl LegoRepo {
        &self.lego_repo
    }

    fn steam_games_repo(&self) -> &impl SteamGamesRepo {
        &self.games_repo
    }

    fn steam_achievements_repo(&self) -> &impl SteamAchievementsRepo {
        &self.game_achievements_repo
    }

    fn movie_reviews_repo(&self) -> &impl MovieReviewsRepo {
        &self.movie_reviews_repo
    }

    fn tv_show_reviews_repo(&self) -> &impl TvShowReviewsRepo {
        &self.tv_show_reviews_repo
    }

    fn albums_repo(&self) -> &impl AlbumsRepo {
        &self.albums_repo
    }

    fn referrals_repo(&self) -> &impl ReferralsRepo {
        &self.referrals_repo
    }

    fn faq_repo(&self) -> &impl FaqRepo {
        &self.faq_repo
    }

    fn now_text_repo(&self) -> &impl NowTextRepo {
        &self.now_text_repo
    }

    fn league_repo(&self) -> &impl LeagueRepo {
        &self.league_repo
    }

    fn omni_post_repo(&self) -> &impl OmniPostRepo {
        &self.omni_post_repo
    }

    fn projects_repo(&self) -> &impl ProjectsRepo {
        &self.projects_repo
    }

    // services

    fn cache_service(&self) -> &impl CacheService {
        &self.cache_service
    }

    fn cdn_service(&self) -> &impl CdnService {
        &self.cdn_service
    }

    fn movie_service(&self) -> &impl MovieService {
        &self.movie_service
    }

    fn image_service(&self) -> &impl ImageService {
        &self.image_service
    }

    fn network_service(&self) -> &impl NetworkService {
        &self.network_service
    }

    fn file_service(&self) -> &impl FileService {
        &self.file_service
    }

    fn query_limiting_service(&self) -> &impl QueryLimitingService {
        &self.query_limiting_service
    }

    fn tv_shows_service(&self) -> &impl TvShowsService {
        &self.tv_shows_service
    }

    fn page_rendering_service(&self) -> &impl PageRenderingService {
        &self.page_rendering_service
    }

    fn book_service(&self) -> &impl BookService {
        &self.book_service
    }
}
