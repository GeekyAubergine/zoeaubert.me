use crate::{
    domain::{repositories::NowTextRepo, state::State},
    prelude::Result,
};

pub struct NowText {
    pub now_text: String,
}

impl NowText {
    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            now_text: state.now_text_repo().find().await?,
        })
    }
}
