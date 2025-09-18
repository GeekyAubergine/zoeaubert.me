use tokio::try_join;
use tracing::info;

use crate::application::commands::album_commands::update_albums_command::update_albums_command;
use crate::application::commands::blog_posts_commands::update_blog_posts_command::update_blog_posts_command;
use crate::application::commands::faq_commands::update_faq_command::update_faq_command;
use crate::application::commands::league_commands::update_league_data_command;
use crate::application::commands::lego_commands::update_lego_command::update_lego_command;
use crate::application::commands::mastodon_posts_commands::update_mastodon_posts_command::update_mastodon_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_blog_archive_posts_command::update_micro_blog_archive_posts_command;
use crate::application::commands::micro_posts_commands::update_micro_posts_command::update_micro_posts;
use crate::application::commands::project_commands::update_projects_command::update_projects_command;
use crate::application::commands::referrals_commands::update_referrals_command::update_referrals_command;
use crate::application::commands::update_derived_data_command::update_derived_data_command;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;
use crate::prelude::*;

use super::silly_names_commands::update_silly_names::update_silly_names_command;

pub async fn update_all_data_command(state: &impl State) -> Result<()> {
    info!("Processing data");

    state.profiler().entity_processing_started().await?;

    try_join!(
        update_silly_names_command(state),
        update_blog_posts_command(state),
        update_micro_blog_archive_posts_command(state),
        update_micro_posts(state),
        update_mastodon_posts_command(state),
        update_lego_command(state),
        update_albums_command(state),
        update_referrals_command(state),
        update_faq_command(state),
        update_league_data_command(state),
        update_projects_command(state),
    )?;

    update_derived_data_command(state).await?;

    state.profiler().entity_processing_finished().await?;

    Ok(())
}
