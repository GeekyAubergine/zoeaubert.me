use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusLolPost {
    id: String,
    date: DateTime<Utc>,
    content: String,
    emoji: String,
    original_url: String,
    updated_at: DateTime<Utc>,
}

impl StatusLolPost {
    pub fn new(
        id: String,
        date: DateTime<Utc>,
        content: String,
        emoji: String,
        original_url: String,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            date,
            content,
            emoji,
            original_url,
            updated_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn permalink(&self) -> String {
        format!("/micros/statuslol-{}", self.id)
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn emoji(&self) -> &str {
        &self.emoji
    }

    pub fn original_url(&self) -> &str {
        &self.original_url
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusLolPosts {
    posts: HashMap<String, StatusLolPost>,
}

impl StatusLolPosts {
    pub fn new() -> Self {
        Self {
            posts: HashMap::new(),
        }
    }

    pub fn add_post(&mut self, post: StatusLolPost) {
        let key = post.id().to_string();
        self.posts.insert(key.clone(), post);
    }

    pub fn add_posts(&mut self, posts: Vec<StatusLolPost>) {
        for post in posts {
            self.add_post(post);
        }
    }

    pub fn get_post(&self, key: &str) -> Option<&StatusLolPost> {
        self.posts.get(key)
    }
}
