use super::repositories::{
    about_text_repo_memory::AboutTextRepoMemory, blog_posts_repo_memory::BlogPostsRepoMemory, silly_names_repo_memory::SillyNamesRepoMemory
};

use crate::{
    domain::{repositories::{AboutTextRepo, BlogPostsRepo, SillyNamesRepo}, state::State},
    prelude::*,
};

pub struct AppState {
    silly_names_repo: SillyNamesRepoMemory,
    about_text_repo: AboutTextRepoMemory,
    blog_posts_repo: BlogPostsRepoMemory,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            silly_names_repo: SillyNamesRepoMemory::new(),
            about_text_repo: AboutTextRepoMemory::new(),
            blog_posts_repo: BlogPostsRepoMemory::new(),
        })
    }
}

impl State for AppState {
    fn silly_names_repo(&self) -> &impl SillyNamesRepo {
        &self.silly_names_repo
    }

    fn about_text_repo(&self) -> &impl AboutTextRepo {
        &self.about_text_repo
    }

    fn blog_posts_repo(&self) -> &impl BlogPostsRepo {
        &self.blog_posts_repo
    }
}
