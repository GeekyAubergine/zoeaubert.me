use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoSet {
    id: u32,
    name: String,
    number: String,
    category: String,
    pieces: u32,
    image: String,
    thumbnail: String,
    link: String,
    quantity: u32,
    updated_at: DateTime<Utc>,
}

impl LegoSet {
    pub fn new(
        id: u32,
        name: String,
        number: String,
        category: String,
        pieces: u32,
        image: String,
        thumbnail: String,
        link: String,
        quantity: u32,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            number,
            category,
            pieces,
            image,
            thumbnail,
            link,
            quantity,
            updated_at,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn pieces(&self) -> u32 {
        self.pieces
    }

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn thumbnail(&self) -> &str {
        &self.thumbnail
    }

    pub fn link(&self) -> &str {
        &self.link
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LegoMinifig {
    id: String,
    name: String,
    category: String,
    owned_in_sets: u32,
    owned_loose: u32,
    total_owned: u32,
    image_url: String,
    updated_at: DateTime<Utc>,
}

impl LegoMinifig {
    pub fn new(
        id: String,
        name: String,
        category: String,
        owned_in_sets: u32,
        owned_loose: u32,
        total_owned: u32,
        image_url: String,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            owned_in_sets,
            owned_loose,
            total_owned,
            image_url,
            updated_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn owned_in_sets(&self) -> u32 {
        self.owned_in_sets
    }

    pub fn owned_loose(&self) -> u32 {
        self.owned_loose
    }

    pub fn total_owned(&self) -> u32 {
        self.total_owned
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn link(&self) -> String {
        format!("https://www.brickset.com/minifigs/{}", self.id)
    }
}
