use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct About {
    short: String,
    long: String,
}

impl About {
    pub fn new(short: String, long: String) -> Self {
        Self { short, long }
    }

    pub fn short(&self) -> &str {
        &self.short
    }

    pub fn long(&self) -> &str {
        &self.long
    }
}
