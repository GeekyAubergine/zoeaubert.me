use crate::prelude::*;

use super::models::{blog_post::BlogPost, mastodon_post::MastodonPost, micro_post::MicroPost};

#[async_trait::async_trait]
pub trait Profiler {
    async fn add_post_processed(&self) -> Result<()>;

    async fn add_page_generated(&self) -> Result<()>;

    async fn overall_start(&self) -> Result<()>;

    async fn overall_stop(&self) -> Result<()>;

    async fn page_generation_start(&self) -> Result<()>;

    async fn print_results(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait SillyNamesRepo {
    async fn find_all(&self) -> Result<Vec<String>>;

    async fn commit(&self, names: Vec<String>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait AboutTextRepo {
    async fn find_short(&self) -> Result<String>;

    async fn find_long(&self) -> Result<String>;

    async fn commit(&self, short: String, long: String) -> Result<()>;
}

#[async_trait::async_trait]
pub trait BlogPostsRepo {
    async fn find_all(&self) -> Result<Vec<BlogPost>>;

    async fn commit(&self, blog_post: &BlogPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MicroPostsRepo {
    async fn find_all(&self) -> Result<Vec<MicroPost>>;

    async fn commit(&self, micro_post: &MicroPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait MastodonPostsRepo {
    async fn find_all(&self) -> Result<Vec<MastodonPost>>;

    async fn commit(&self, micro_post: &MastodonPost) -> Result<()>;
}
