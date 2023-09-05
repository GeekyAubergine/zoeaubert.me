use crate::{error::Error, load_data_from_file, prelude::*};

use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

const CACHE_FILE: &str = ".cache/mastodon.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPost {
    id: u32,
    permalink: String,
    date: DateTime<Utc>,
    content: String,
    tags: Vec<String>,
    description: String,
}

impl MastodonPost {
    pub fn new(
        id: u32,
        permalink: String,
        date: DateTime<Utc>,
        content: String,
        tags: Vec<String>,
        description: String,
    ) -> Self {
        Self {
            id,
            permalink,
            date,
            content,
            tags,
            description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastodonPosts {
    posts: HashMap<u32, MastodonPost>,
}

impl MastodonPosts {
    pub fn new() -> Self {
        Self {
            posts: HashMap::new(),
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        match load_data_from_file(CACHE_FILE).await {
            Ok(data) => {
                let posts: HashMap<u32, MastodonPost> = serde_json::from_slice(&data)
                    .map_err(|e| Error::UnableToParseMastodonPostsCache(e.to_string()))?;

                self.posts = posts;

                Ok(())
            }
            Err(Error::UnableToFindFile(e)) => {
                Ok(())
            }
            Err(e) => Err(e),
        }

    }

    pub fn add(&mut self, post: MastodonPost) {
        self.posts.insert(post.id, post);
    }

    pub fn update(&mut self, post: MastodonPost) {
        self.posts.insert(post.id, post);
    }

    pub fn get(&self, id: &u32) -> Option<&MastodonPost> {
        self.posts.get(id)
    }

    pub fn get_all(&self) -> &HashMap<u32, MastodonPost> {
        &self.posts
    }
}

pub type MastodonPostsRepo = Arc<Mutex<MastodonPosts>>;

pub fn new_mastodon_posts_repo() -> MastodonPostsRepo {
    Arc::new(Mutex::new(MastodonPosts::new()))
}
