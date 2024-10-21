use super::{
    repositories::{AboutTextRepo, BlogPostsRepo, GameAchievementsRepo, GamesRepo, LegoRepo, MastodonPostsRepo, MicroPostsRepo, Profiler, SillyNamesRepo},
    services::{CacheService, CdnService},
};

pub trait State {
    fn profiler(&self) -> &impl Profiler;

    fn silly_names_repo(&self) -> &impl SillyNamesRepo;

    fn about_text_repo(&self) -> &impl AboutTextRepo;

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo;

    fn micro_posts_repo(&self) -> &impl MicroPostsRepo;

    fn mastodon_posts_repo(&self) -> &impl MastodonPostsRepo;

    fn lego_repo(&self) -> &impl LegoRepo;

    fn games_repo(&self) -> &impl GamesRepo;

    fn game_achievements_repo(&self) -> &impl GameAchievementsRepo;

    fn cache_service(&self) -> &impl CacheService;

    fn cdn_service(&self) -> &impl CdnService;
}
