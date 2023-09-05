use crate::prelude::*;

use axum_macros::FromRef;
use chrono::{DateTime, Utc};

use crate::config::{ConfigRepo};

use self::mastodon_post::{new_mastodon_posts_repo, MastodonPostsRepo};

pub mod mastodon_post;

pub trait Post {
    fn key(&self) -> &str;
    fn permalink(&self) -> &str;
    fn date(&self) -> &DateTime<Utc>;
    fn content(&self) -> &str;
    fn tags(&self) -> &Vec<String>;
    fn description(&self) -> &str;
}

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    config: ConfigRepo,
    mastodon_posts: MastodonPostsRepo,
}

impl AppState {
    pub fn new(config: ConfigRepo) -> Self {
        Self {
            config,
            mastodon_posts: new_mastodon_posts_repo(),
        }
    }

    pub async fn init(&self) -> Result<()> {
        let mut mastodon_posts = self.mastodon_posts.lock().await;

        mastodon_posts.init().await?;

        Ok(())
    }
}
