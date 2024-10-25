use tokio::try_join;
use tracing::info;

use crate::domain::queries::omni_post_queries::{
    find_all_omni_posts, find_all_omni_posts_by_tag, OmniPostFilterFlags,
};
use crate::domain::queries::tags_queries::find_tag_counts;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;

use crate::infrastructure::renderers::blog_pages::render_blog_pages;
use crate::infrastructure::renderers::games_pages::render_games_pages;
use crate::infrastructure::renderers::home_page::render_home_page;
use crate::infrastructure::renderers::interests_page::render_interests_list_page;
use crate::infrastructure::renderers::lego_pages::render_lego_page;
use crate::infrastructure::renderers::mastodon_post_pages::render_mastodon_post_pages;
use crate::infrastructure::renderers::micro_post_pages::render_micro_post_pages;
use crate::infrastructure::renderers::movie_pages::render_movie_pages;
use crate::infrastructure::renderers::tags_pages::render_tags_pages;
use crate::infrastructure::renderers::timeline_pages::render_timeline_page;
use crate::infrastructure::renderers::tv_show_pages::render_tv_show_pages;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_site(state: &impl State) -> Result<()> {
    info!("Building site");
    state.profiler().page_generation_started().await?;

    let home_page = render_home_page(state);

    try_join!(
        render_home_page(state),
        // Posts
        render_blog_pages(state),
        render_micro_post_pages(state),
        render_mastodon_post_pages(state),
        // Timeline and tags
        render_timeline_page(state),
        render_tags_pages(state),
        render_interests_pages(state),
    )?;

    state.profiler().page_generation_finished().await?;
    state.profiler().print_results().await?;

    Ok(())
}

async fn render_interests_pages(state: &impl State) -> Result<()> {
    try_join!(
        render_interests_list_page(state),
        render_lego_page(state),
        render_games_pages(state),
        render_movie_pages(state),
    )?;

    render_tv_show_pages(state).await?;

    Ok(())
}
