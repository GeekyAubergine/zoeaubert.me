use crate::domain::repositories::SillyNamesRepo;
use crate::domain::state::State;
use crate::prelude::Result;

pub struct SillyNames {
    pub silly_names: Vec<String>,
}

impl SillyNames {
    pub fn new(silly_names: Vec<String>) -> Self {
        Self { silly_names }
    }

    pub async fn from_state(state: &impl State) -> Result<Self> {
        Ok(Self {
            silly_names: state.silly_names_repo().find_all().await?,
        })
    }
}
