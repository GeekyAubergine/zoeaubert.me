use crate::domain::repositories::SillyNamesRepo;

use crate::domain::state::State;
use crate::prelude::*;

pub async fn find_silly_names(state: &impl State) -> Result<Vec<String>> {
    state.silly_names_repo().find_all().await
}

pub async fn commit_silly_names(state: &impl State, silly_names: Vec<String>) -> Result<()> {
    state.silly_names_repo().commit(silly_names).await
}
