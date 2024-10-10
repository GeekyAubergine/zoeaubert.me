use super::repositories::{AboutTextRepo, BlogPostsRepo, SillyNamesRepo};

pub trait State {
    fn silly_names_repo(&self) -> &impl SillyNamesRepo;

    fn about_text_repo(&self) -> &impl AboutTextRepo;

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo;
}
