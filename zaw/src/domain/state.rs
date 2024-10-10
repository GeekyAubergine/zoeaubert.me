use super::repositories::{AboutTextRepo, BlogPostsRepo, Profiler, SillyNamesRepo};

pub trait State {
    fn profiler(&self) -> &impl Profiler;

    fn silly_names_repo(&self) -> &impl SillyNamesRepo;

    fn about_text_repo(&self) -> &impl AboutTextRepo;

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo;
}
