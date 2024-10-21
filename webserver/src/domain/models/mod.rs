use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod about;
pub mod album;
pub mod blog_post;
pub mod faq;
pub mod game;
pub mod game_achievement;
pub mod lego;
pub mod mastodon_post;
pub mod media;
pub mod micro_post;
pub mod now;
pub mod omni_post;
pub mod page;
pub mod status_lol_post;
pub mod tag;
pub mod silly_name;

pub trait UuidIdentifiable {
    fn uuid(&self) -> &Uuid;
}
