use tracing::info;

use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::queries::games_queries::find_all_games;
use crate::domain::queries::mastodon_queries::find_all_mastodon_posts;
use crate::domain::queries::micro_posts_queries::find_all_micro_posts;
use crate::domain::queries::omni_post_queries::{
    find_all_omni_posts, find_all_omni_posts_by_tag, OmniPostFilterFlags,
};
use crate::domain::queries::tags_queries::find_tag_counts;
use crate::domain::repositories::Profiler;
use crate::domain::state::State;

use crate::infrastructure::renderers::blog_pages::{render_blog_post_page, render_blogs_list_page};
use crate::infrastructure::renderers::games_pages::{render_game_page, render_games_list_page};
use crate::infrastructure::renderers::home_page::render_home_page;
use crate::infrastructure::renderers::interests_page::render_interests_list_page;
use crate::infrastructure::renderers::lego_pages::render_lego_list_page;
use crate::infrastructure::renderers::mastodon_post_pages::render_mastodon_post_page;
use crate::infrastructure::renderers::micro_post_pages::render_micro_post_page;
use crate::infrastructure::renderers::tags_pages::{render_tag_page, render_tags_list_page};
use crate::infrastructure::renderers::timeline_pages::render_timeline_page;
use crate::infrastructure::utils::paginator::paginate;
use crate::prelude::*;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn build_site(state: &impl State) -> Result<()> {
    info!("Building site");
    state.profiler().page_generation_started().await?;
    render_home_page(state).await?;

    build_blog_pages(state).await?;

    build_timeline_pages(state).await?;

    build_tags_pages(state).await?;

    build_micro_post_pages(state).await?;

    build_mastodon_post_pages(state).await?;

    build_interests_pages(state).await?;

    state.profiler().page_generation_finished().await?;
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

async fn build_micro_post_pages(state: &impl State) -> Result<()> {
    let micro_posts = find_all_micro_posts(state).await?;

    for micro_post in micro_posts {
        render_micro_post_page(state, &micro_post).await?;
    }

    Ok(())
}

async fn build_mastodon_post_pages(state: &impl State) -> Result<()> {
    let mastodon_posts = find_all_mastodon_posts(state).await?;

    for mastodon_post in mastodon_posts {
        render_mastodon_post_page(state, &mastodon_post).await?;
    }

    Ok(())
}

async fn build_interests_pages(state: &impl State) -> Result<()> {
    render_interests_list_page(state).await?;

    build_lego_pages(state).await?;
    build_games_pages(state).await?;
    Ok(())
}

async fn build_lego_pages(state: &impl State) -> Result<()> {
    render_lego_list_page(state).await
}

async fn build_games_pages(state: &impl State) -> Result<()> {
    let games = find_all_games(state).await?;

    render_games_list_page(state, &games).await?;

    for game in games {
        render_game_page(state, &game).await?;
    }

    Ok(())
}
