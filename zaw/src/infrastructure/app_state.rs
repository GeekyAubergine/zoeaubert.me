use super::{
    repositories::{
        about_text_repo_memory::AboutTextRepoMemory, blog_posts_repo_memory::BlogPostsRepoMemory, lego_repo_disk::LegoRepoDisk, mastodon_post_repo_disk::MastodonPostRepoDisk, micro_blog_repo_memory::MicroPostsRepoMemory, profiler_memory::ProfilerMemory, silly_names_repo_memory::SillyNamesRepoMemory
    },
    services::{cache_service_disk::CacheServiceDisk, cdn_service_bunny::CdnServiceBunny},
};

use crate::{
    domain::{
        repositories::{AboutTextRepo, BlogPostsRepo, LegoRepo, MastodonPostsRepo, MicroPostsRepo, Profiler, SillyNamesRepo},
        services::{CacheService, CdnService},
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
    mastodon_posts_repo: MastodonPostRepoDisk,
    lego_repo: LegoRepoDisk,
    cache_service: CacheServiceDisk,
    cdn_service: CdnServiceBunny,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            profiler: ProfilerMemory::new(),
            silly_names_repo: SillyNamesRepoMemory::new(),
            about_text_repo: AboutTextRepoMemory::new(),
            blog_posts_repo: BlogPostsRepoMemory::new(),
            micro_posts_repo: MicroPostsRepoMemory::new(),
            mastodon_posts_repo: MastodonPostRepoDisk::new().await?,
            lego_repo: LegoRepoDisk::new().await?,
            cache_service: CacheServiceDisk::new(),
            cdn_service: CdnServiceBunny::new(),
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

    fn cache_service(&self) -> &impl CacheService {
        &self.cache_service
    }

    fn cdn_service(&self) -> &impl CdnService {
        &self.cdn_service
    }
}
