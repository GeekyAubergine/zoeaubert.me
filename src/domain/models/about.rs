use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct About {
    text: String,
}

impl About {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}