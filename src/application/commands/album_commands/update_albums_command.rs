use std::path::Path;

use serde::Deserialize;

use crate::{
    application::commands::album_commands::update_album_command::update_album_command,
    domain::{services::FileService, state::State},
    prelude::*,
};

const ALBUMS_POSTS_DIR: &str = "albums";

pub async fn update_albums_command(state: &impl State) -> Result<()> {
    let path = state
        .file_service()
        .make_content_file_path(&Path::new(ALBUMS_POSTS_DIR));
    let albums = state
        .file_service()
        .find_files_rescurse(&path, "yml")
        .await?;

    for album in albums {
        let file_path = Path::new(&album);
        update_album_command(state, &file_path).await?;
    }

    Ok(())
}
