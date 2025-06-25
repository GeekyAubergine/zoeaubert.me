use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    prelude::*,
};

use super::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoSet {
    pub id: u32,
    pub name: String,
    pub number: String,
    pub category: String,
    pub pieces: u32,
    pub image: Image,
    pub thumbnail: Image,
    pub link: Url,
    pub quantity: u32,
}

impl LegoSet {
    pub fn new(
        id: u32,
        name: String,
        number: String,
        category: String,
        pieces: u32,
        image: Image,
        thumbnail: Image,
        link: Url,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegoMinifig {
    pub id: String,
    pub name: String,
    pub category: String,
    pub owned_in_sets: u32,
    pub owned_loose: u32,
    pub total_owned: u32,
    pub image: Image,
}

impl LegoMinifig {
    pub fn new(
        id: String,
        name: String,
        category: String,
        owned_in_sets: u32,
        owned_loose: u32,
        total_owned: u32,
        image: Image,
    ) -> Self {
        Self {
            id,
            name,
            category,
            owned_in_sets,
            owned_loose,
            total_owned,
            image,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Lego {
    sets: HashMap<u32, LegoSet>,
    minifigs: HashMap<String, LegoMinifig>,
}

impl Lego {
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
            minifigs: HashMap::new(),
        }
    }

    pub fn find_all_sets(&self) -> Vec<&LegoSet> {
        let mut sets = self.sets.values().collect::<Vec<&LegoSet>>();

        sets.sort_by(|a, b| b.pieces.cmp(&a.pieces));

        sets
    }

    pub fn find_all_minifigs(&self) -> Vec<&LegoMinifig> {
        let mut minifigs = self.minifigs.values().collect::<Vec<&LegoMinifig>>();

        minifigs.sort_by(|a, b| a.name.cmp(&b.name));

        minifigs
    }

    pub fn find_total_pieces(&self) -> u32 {
        self.sets.values().map(|set| set.pieces).sum()
    }

    pub fn find_total_sets(&self) -> u32 {
        self.sets.len() as u32
    }

    pub fn find_total_minifigs(&self) -> u32 {
        self.minifigs.len() as u32
    }

    pub fn add_set(&mut self, set: &LegoSet) {
        self.sets.insert(set.id, set.clone());
    }

    pub fn add_minifig(&mut self, minifig: &LegoMinifig) {
        self.minifigs.insert(minifig.id.clone(), minifig.clone());
    }
}
