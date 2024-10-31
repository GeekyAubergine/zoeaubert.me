use std::sync::Arc;

use super::{
    repositories::{
        about_text_repo_memory::AboutTextRepoMemory, albums_repo_disk::AlbumsRepoDisk, blog_posts_repo_disk::BlogPostsRepoDisk, game_achievements_repo_disk::GameAchievementsRepoDisk, games_repo_disk::GamesRepoDisk, lego_repo_disk::LegoRepoDisk, mastodon_post_repo_disk::MastodonPostRepoDisk, micro_blog_repo_disk::MicroPostsRepoDisk, movie_reviews_repo_memory::MovieReviewsRepoMemory, profiler_memory::ProfilerMemory, referrary_repo_memory::ReferralsRepoMemory, silly_names_repo_memory::SillyNamesRepoMemory, tv_show_reviews_repo_memory::TvShowReviewsRepoMemory
    },
    services::{
        cache_service_disk::CacheServiceDisk, cdn_service_bunny::CdnServiceBunny,
        file_service_disk::FileServiceDisk, image_service_impl::ImageServiceImpl,
        movie_service_tmdb::MovieServiceTmdb, network_service_reqwest::NetworkServiceReqwest,
        query_limiting_service_disk::QueryLimitingServiceDisk,
        tv_shows_service_tmdb::TvShowsServiceTmdb,
    },
};

use crate::{
    domain::{
        repositories::{
            AboutTextRepo, AlbumsRepo, BlogPostsRepo, GameAchievementsRepo, GamesRepo, LegoRepo, MastodonPostsRepo, MicroPostsRepo, MovieReviewsRepo, Profiler, ReferralsRepo, SillyNamesRepo, TvShowReviewsRepo
        },
        services::{
            CacheService, CdnService, FileService, ImageService, MovieService, NetworkService,
            QueryLimitingService, TvShowsService,
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
    cache_service: CacheServiceDisk,
    cdn_service: CdnServiceBunny,
    movie_service: MovieServiceTmdb,
    image_service: ImageServiceImpl,
    network_service: NetworkServiceReqwest,
    file_service: FileServiceDisk,
    query_limiting_service: QueryLimitingServiceDisk,
    tv_shows_service: TvShowsServiceTmdb,
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
            cache_service: CacheServiceDisk::new(),
            cdn_service: CdnServiceBunny::new(),
            movie_service: MovieServiceTmdb::new().await?,
            image_service: ImageServiceImpl::new(),
            network_service: NetworkServiceReqwest::new(),
            file_service: FileServiceDisk::new(),
            query_limiting_service: QueryLimitingServiceDisk::new().await?,
            tv_shows_service: TvShowsServiceTmdb::new().await?,
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

    fn games_repo(&self) -> &impl GamesRepo {
        &self.games_repo
    }

    fn game_achievements_repo(&self) -> &impl GameAchievementsRepo {
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
}
