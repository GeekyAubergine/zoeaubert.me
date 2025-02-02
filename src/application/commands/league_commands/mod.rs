use tokio::try_join;

use crate::prelude::*;

use crate::domain::state::State;

use update_league_champ_notes_command::update_league_champ_notes_command;

pub mod update_league_champ_notes_command;

pub async fn update_league_data_command(state: &impl State) -> Result<()> {
    try_join!(update_league_champ_notes_command(state),)?;

    Ok(())
}
