use tokio::try_join;
use tracing::info;

use crate::application::commands::about_text_commands::update_about_text_command::{
    self, update_about_text_command,
};
use crate::application::commands::blog_posts_commands::update_blog_posts_command::update_blog_posts_command;
use crate::application::commands::games_commands::update_games_command::update_games_command;
use crate::application::commands::lego_commands::update_lego_command::update_lego_command;
use crate::application::commands::mastodon_posts_commands::update_mastodon_posts_command::update_mastodon_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_blog_archive_posts_command::update_micro_blog_archive_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_posts_command::update_micro_posts;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;
use crate::prelude::*;

use super::silly_names_commands::update_silly_names::update_silly_names_command;

pub async fn update_all_data_command(state: &impl State) -> Result<()> {
    info!("Processing data");

    state.profiler().post_processing_started().await?;

    try_join!(
        update_silly_names_command(state),
        update_about_text_command(state),
        update_blog_posts_command(state),
        update_micro_blog_archive_posts_command(state),
        update_micro_posts(state),
        update_mastodon_posts_command(state),
        update_lego_command(state),
        update_games_command(state),
    )?;

    state.profiler().post_processing_finished().await?;

    Ok(())
}
