use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoSet {
    pub id: u32,
    pub name: String,
    pub number: String,
    pub category: String,
    pub pieces: u32,
    pub image: String,
    pub thumbnail: String,
    pub link: String,
    pub quantity: u32,
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
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LegoMinifig {
    pub id: String,
    pub name: String,
    pub category: String,
    pub owned_in_sets: u32,
    pub owned_loose: u32,
    pub total_owned: u32,
    pub image_url: String,
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
    ) -> Self {
        Self {
            id,
            name,
            category,
            owned_in_sets,
            owned_loose,
            total_owned,
            image_url,
        }
    }

    pub fn display_name(&self) -> String {
        let name = match self.name.split(" - ").next() {
            Some(name) => name.to_string(),
            None => self.name.clone(),
        };

        name.replace("(Minifigure Only without Stand and Accessories)", "")
    }

    pub fn link(&self) -> String {
        format!("https://www.brickset.com/minifigs/{}", self.id)
    }
}
