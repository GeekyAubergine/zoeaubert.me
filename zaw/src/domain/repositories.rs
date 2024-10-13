use chrono::{DateTime, Utc};

use crate::prelude::*;

use super::models::{blog_post::BlogPost, lego::{LegoMinifig, LegoSet}, mastodon_post::MastodonPost, micro_post::MicroPost};

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

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit(&self, micro_post: &MastodonPost) -> Result<()>;
}

#[async_trait::async_trait]
pub trait LegoRepo {
    async fn find_all_sets(&self) -> Result<Vec<LegoSet>>;

    async fn find_all_minifigs(&self) -> Result<Vec<LegoMinifig>>;

    async fn find_total_pieces(&self) -> Result<u32>;

    async fn find_total_sets(&self) -> Result<u32>;

    async fn find_total_minifigs(&self) -> Result<u32>;

    async fn find_last_updated_at(&self) -> Result<Option<DateTime<Utc>>>;

    async fn commit_set(&self, set: &LegoSet) -> Result<()>;

    async fn commit_minifig(&self, minifig: &LegoMinifig) -> Result<()>;
}
