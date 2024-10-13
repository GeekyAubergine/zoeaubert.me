use tracing::info;

use crate::application::commands::about_text_commands::update_about_text;
use crate::application::commands::blog_posts_commands::update_blog_posts_command::update_blog_posts_command;
use crate::application::commands::mastodon_posts_commands::update_mastodon_posts_command::update_mastodon_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_blog_archive_posts_command::update_micro_blog_archive_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_posts_command::update_micro_posts;
use crate::domain::state::State;
use crate::prelude::*;

use super::silly_names_commands::update_silly_names::update_silly_names;

pub async fn update_all_data(state: &impl State) -> Result<()> {
    info!("Updating all data");

    update_silly_names(state).await?;
    update_about_text(state).await?;
    update_blog_posts_command(state).await?;
    update_micro_blog_archive_posts_command(state).await?;
    update_micro_posts(state).await?;
    update_mastodon_posts_command(state).await?;

    Ok(())
}
