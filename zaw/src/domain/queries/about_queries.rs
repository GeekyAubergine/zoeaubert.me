use crate::domain::repositories::AboutTextRepo;
use crate::domain::state::State;

use crate::prelude::*;

pub async fn find_about_text_short(state: &impl State) -> Result<String> {
    state.about_text_repo().find_short().await
}

pub async fn find_about_text_long(state: &impl State) -> Result<String> {
    state.about_text_repo().find_long().await
}
