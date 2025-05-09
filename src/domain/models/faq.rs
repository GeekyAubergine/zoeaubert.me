use crate::{
    domain::{repositories::FaqRepo, state::State},
    prelude::Result,
};

pub struct Faq {
    pub faq: String,
}

impl Faq {
    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            faq: state.faq_repo().find().await?,
        })
    }
}
