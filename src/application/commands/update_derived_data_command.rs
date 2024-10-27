use tracing::warn;

use tokio::try_join;

use crate::{domain::{models::tag::Tag, queries::omni_post_queries::find_all_omni_posts_by_tag, repositories::{MovieReviewsRepo, Profiler, TvShowReviewsRepo}, services::{MovieService, TvShowsService}, state::State}, prelude::*};

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";

async fn update_movie_reviews(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts_by_tag(state, &Tag::from_string(MOVIE_REVIEW_POST_TAG)).await?;

    for post in posts {
        state.profiler().post_processed().await?;

        match state.movie_service().movie_review_from_omni_post(state, &post).await {
            Ok(review) => state.movie_reviews_repo().commit(&review).await?,
            Err(e) => warn!(
                "Could not create movie review from post with slug: {} {:?}",
                post.slug(),
                e,
            )
        };
    }

    Ok(())
}

async fn update_tv_reviews(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts_by_tag(state, &Tag::from_string(TV_SHOW_REVIEW_POST_TAG)).await?;

    for post in posts {
        state.profiler().post_processed().await?;

        match state.tv_shows_service().tv_show_review_from_omni_post(state, &post).await {
            Ok(review) => state.tv_show_reviews_repo().commit(&review).await?,
            Err(e) => warn!(
                "Could not create tv review from post with slug: {} {:?}",
                post.slug(),
                e,
            )
        };
    }

    Ok(())
}

pub async fn update_derived_data_command(state: &impl State) -> Result<()> {
    try_join!(
        update_movie_reviews(state),
        update_tv_reviews(state),
    )?;

    Ok(())
}
