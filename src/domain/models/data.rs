use crate::domain::repositories::AboutTextRepo;
use crate::prelude::*;

use crate::domain::state::State;

use super::about_text::AboutText;
use super::silly_names::SillyNames;

pub struct Data {
    pub about_text: AboutText,
    pub silly_names: SillyNames
}

impl Data {
    pub async fn from_state(state: &impl State) -> Result<Data> {
        Ok(Data {
            about_text: AboutText::from_state(state).await?,
            silly_names: SillyNames::from_state(state).await?,
        })
    }
}
