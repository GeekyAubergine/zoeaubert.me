use super::{
    repositories::{AboutTextRepo, BlogPostsRepo, MicroPostsRepo, Profiler, SillyNamesRepo},
    services::CacheService,
};

pub trait State {
    fn profiler(&self) -> &impl Profiler;

    fn silly_names_repo(&self) -> &impl SillyNamesRepo;

    fn about_text_repo(&self) -> &impl AboutTextRepo;

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo;

    fn micro_posts_repo(&self) -> &impl MicroPostsRepo;

    fn cache_service(&self) -> &impl CacheService;
}
