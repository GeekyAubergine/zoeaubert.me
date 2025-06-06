use crate::prelude::Result;

use crate::domain::{repositories::AboutTextRepo, state::State};

pub struct AboutText {
    pub short: String,
    pub long: String,
}

impl AboutText {
    pub fn new(short: String, long: String) -> Self {
        Self { short, long }
    }

    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            short: state.about_text_repo().find_short().await?,
            long: state.about_text_repo().find_long().await?,
        })
    }
}
