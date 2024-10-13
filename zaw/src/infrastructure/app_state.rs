use super::{
    repositories::{
        about_text_repo_memory::AboutTextRepoMemory, blog_posts_repo_memory::BlogPostsRepoMemory, micro_blog_repo_memory::MicroPostsRepoMemory, profiler_memory::ProfilerMemory, silly_names_repo_memory::SillyNamesRepoMemory
    },
    services::cache_service_disk::CacheServiceDisk,
};

use crate::{
    domain::{
        repositories::{AboutTextRepo, BlogPostsRepo, MicroPostsRepo, Profiler, SillyNamesRepo},
        services::CacheService,
        state::State,
    },
    prelude::*,
};

pub struct AppState {
    profiler: ProfilerMemory,
    silly_names_repo: SillyNamesRepoMemory,
    about_text_repo: AboutTextRepoMemory,
    blog_posts_repo: BlogPostsRepoMemory,
    micro_posts_repo: MicroPostsRepoMemory,
    cache_service: CacheServiceDisk,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            profiler: ProfilerMemory::new(),
            silly_names_repo: SillyNamesRepoMemory::new(),
            about_text_repo: AboutTextRepoMemory::new(),
            blog_posts_repo: BlogPostsRepoMemory::new(),
            micro_posts_repo: MicroPostsRepoMemory::new(),
            cache_service: CacheServiceDisk::new(),
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

    fn cache_service(&self) -> &impl CacheService {
        &self.cache_service
    }

    fn micro_posts_repo(&self) -> &impl MicroPostsRepo {
        &self.micro_posts_repo
    }
}
