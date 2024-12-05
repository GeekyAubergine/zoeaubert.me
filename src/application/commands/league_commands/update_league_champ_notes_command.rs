use std::path::Path;

use serde::Deserialize;

use crate::domain::models::league::LeagueChampNote;
use crate::domain::repositories::LeagueRepo;
use crate::domain::services::FileService;
use crate::prelude::*;

use crate::domain::state::State;

const FILE_NAME: &str = "wiki/league_champ_notes.yml";

#[derive(Debug, Clone, Deserialize)]
struct LeagueChampNotesFile {
    pub champs: Vec<LeagueChampNote>,
}

pub async fn update_league_champ_notes_command(state: &impl State) -> Result<()> {
    let notes: LeagueChampNotesFile = state
        .file_service()
        .read_yaml_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(FILE_NAME)),
        )
        .await?;

    state.league_repo().commit_champ_notes(notes.champs).await?;

    Ok(())
}
