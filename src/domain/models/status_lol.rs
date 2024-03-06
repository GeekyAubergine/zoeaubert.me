use std::collections::HashMap;

use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusLolPost {
    key: String,
    permalink: String,
    date: DateTime<chrono::Utc>,
    content: String,
    emoji: String,
    original_url: String,
}

impl StatusLolPost {
    pub fn new(
        key: String,
        permalink: String,
        date: chrono::DateTime<chrono::Utc>,
        content: String,
        emoji: String,
        original_url: String,
    ) -> Self {
        Self {
            key,
            permalink,
            date,
            content,
            emoji,
            original_url,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn permalink(&self) -> &str {
        &self.permalink
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
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
        let key = post.key().to_string();
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

