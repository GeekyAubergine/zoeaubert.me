use tracing::info;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::queries::omni_post_queries::{
    find_all_omni_posts, find_all_omni_posts_by_tag, OmniPostFilterFlags,
};
use crate::domain::queries::tags_queries::find_tag_counts;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;

use crate::infrastructure::renderers::blog_pages::{render_blog_post_page, render_blogs_list_page};
use crate::infrastructure::renderers::home_page::render_home_page;
use crate::infrastructure::renderers::tags_pages::{render_tag_page, render_tags_list_page};
use crate::infrastructure::renderers::timeline_pages::render_timeline_page;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn build_site(state: &impl State) -> Result<()> {
    info!("Building site");
    render_home_page(state).await?;

    build_blog_pages(state).await?;

    build_timeline_pages(state).await?;

    build_tags_pages(state).await?;

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
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_main_timeline()).await?;

    let paginated = paginate(&omni_posts, DEFAULT_PAGINATION_SIZE);

    for page in paginated {
        render_timeline_page(state, &page).await?;
    }

    Ok(())
}

async fn build_tags_pages(state: &impl State) -> Result<()> {
    let tags = find_tag_counts(state).await?;

    render_tags_list_page(state, &tags).await?;

    for tag in tags.keys() {
        let posts = find_all_omni_posts_by_tag(state, tag).await?;

        let paginated = paginate(&posts, DEFAULT_PAGINATION_SIZE);

        for page in paginated {
            render_tag_page(state, tag, &page).await?;
        }
    }

    Ok(())
}
