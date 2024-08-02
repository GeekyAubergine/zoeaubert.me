use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    name: String,
    header_image_url: String,
    playtime: u32,
    last_played: DateTime<Utc>,
    link_url: String,
    updated_at: DateTime<Utc>,
}

impl Game {
    pub fn new(
        id: u32,
        name: String,
        header_image_url: String,
        playtime: u32,
        last_played: DateTime<Utc>,
        link_url: String,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            header_image_url,
            playtime,
            last_played,
            link_url,
            updated_at,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn header_image_url(&self) -> &str {
        &self.header_image_url
    }

    pub fn playtime(&self) -> u32 {
        self.playtime
    }

    pub fn playtime_hours(&self) -> f32 {
        self.playtime() as f32 / 60.0
    }

    pub fn last_played(&self) -> &DateTime<Utc> {
        &self.last_played
    }

    pub fn link_url(&self) -> &str {
        &self.link_url
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
