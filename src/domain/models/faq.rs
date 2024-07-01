use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Faq {
    text: String,
}

impl Faq {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
