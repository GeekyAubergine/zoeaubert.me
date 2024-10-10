use tracing::info;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;

use crate::infrastructure::renderers::blog_pages::{render_blog_post_page, render_blogs_list_page};
use crate::infrastructure::renderers::home_page::render_home_page;
use crate::infrastructure::renderers::timeline_pages::render_timeline_page;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

pub async fn build_site(state: &impl State) -> Result<()> {
    info!("Building site");
    render_home_page(state).await?;

    build_blog_pages(state).await?;

    build_timeline_pages(state).await?;

    state.profiler().stop_timer().await?;

    state.profiler().print_results().await?;

    Ok(())
}

async fn build_blog_pages(state: &impl State) -> Result<()> {
    let blog_posts = find_all_blog_posts(state).await?;

    render_blogs_list_page(state, &blog_posts).await?;

    for blog_post in blog_posts {
        render_blog_post_page(state, &blog_post).await?;
    }

    Ok(())
}

async fn build_timeline_pages(state: &impl State) -> Result<()> {
    let blog_posts = find_all_blog_posts(state)
        .await?
        .iter()
        .map(|p| p.into())
        .collect::<Vec<OmniPost>>();

    let mut omni_posts = Vec::new();

    omni_posts.extend(blog_posts);

    let paginated = paginate(&omni_posts, 5);

    for page in paginated {
        render_timeline_page(state, &page).await?;
    }

    Ok(())
}
