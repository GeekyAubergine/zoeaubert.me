use super::{
    repositories::{
        AboutTextRepo, AlbumsRepo, BlogPostsRepo, FaqRepo, LeagueRepo, LegoRepo, MastodonPostsRepo, MicroPostsRepo, MovieReviewsRepo, NowTextRepo, OmniPostRepo, Profiler, ReferralsRepo, SillyNamesRepo, SteamAchievementsRepo, SteamGamesRepo, TvShowReviewsRepo
    },
    services::{
        CacheService, CdnService, FileService, ImageService, MovieService, NetworkService,
        PageRenderingService, QueryLimitingService, TvShowsService,
    },
};

pub trait State: Sync + Send {
    fn profiler(&self) -> &impl Profiler;

    fn silly_names_repo(&self) -> &impl SillyNamesRepo;

    fn about_text_repo(&self) -> &impl AboutTextRepo;

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo;

    fn micro_posts_repo(&self) -> &impl MicroPostsRepo;

    fn mastodon_posts_repo(&self) -> &impl MastodonPostsRepo;

    fn lego_repo(&self) -> &impl LegoRepo;

    fn steam_games_repo(&self) -> &impl SteamGamesRepo;

    fn steam_achievements_repo(&self) -> &impl SteamAchievementsRepo;

    fn movie_reviews_repo(&self) -> &impl MovieReviewsRepo;

    fn tv_show_reviews_repo(&self) -> &impl TvShowReviewsRepo;

    fn albums_repo(&self) -> &impl AlbumsRepo;

    fn referrals_repo(&self) -> &impl ReferralsRepo;

    fn faq_repo(&self) -> &impl FaqRepo;

    fn now_text_repo(&self) -> &impl NowTextRepo;

    fn league_repo(&self) -> &impl LeagueRepo;

    fn omni_post_repo(&self) -> &impl OmniPostRepo;

    // --------

    fn cache_service(&self) -> &impl CacheService;

    fn cdn_service(&self) -> &impl CdnService;

    fn movie_service(&self) -> &impl MovieService;

    fn image_service(&self) -> &impl ImageService;

    fn network_service(&self) -> &impl NetworkService;

    fn file_service(&self) -> &impl FileService;

    fn query_limiting_service(&self) -> &impl QueryLimitingService;

    fn tv_shows_service(&self) -> &impl TvShowsService;

    fn page_rendering_service(&self) -> &impl PageRenderingService;
}
