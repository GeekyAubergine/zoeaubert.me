use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoSet {
    key: u32,
    name: String,
    number: String,
    category: String,
    pieces: u32,
    image: String,
    thumbnail: String,
    link: String,
    quantity: u32,
}

impl LegoSet {
    pub fn new(
        key: u32,
        name: String,
        number: String,
        category: String,
        pieces: u32,
        image: String,
        thumbnail: String,
        link: String,
        quantity: u32,
    ) -> Self {
        Self {
            key,
            name,
            number,
            category,
            pieces,
            image,
            thumbnail,
            link,
            quantity,
        }
    }

    pub fn key(&self) -> u32 {
        self.key
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LegoMinifig {
    key: String,
    name: String,
    category: String,
    owned_in_sets: u32,
    owned_loose: u32,
    total_owned: u32,
    image_url: String,
}

impl LegoMinifig {
    pub fn new(
        key: String,
        name: String,
        category: String,
        owned_in_sets: u32,
        owned_loose: u32,
        total_owned: u32,
        image_url: String,
    ) -> Self {
        Self {
            key,
            name,
            category,
            owned_in_sets,
            owned_loose,
            total_owned,
            image_url,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
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
}
