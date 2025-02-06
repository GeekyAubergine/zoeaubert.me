use tracing::warn;

use tokio::try_join;

use crate::{
    domain::{
        models::{content::Content, omni_post::OmniPost, tag::Tag},
        queries::omni_post_queries::find_all_omni_posts_by_tag,
        repositories::{
            AlbumsRepo, BlogPostsRepo, MastodonPostsRepo, MicroPostsRepo, MovieReviewsRepo,
            OmniPostRepo, Profiler, SteamAchievementsRepo, SteamGamesRepo, TvShowReviewsRepo,
        },
        services::{BookService, MovieService, TvShowsService},
        state::State,
    },
    prelude::*,
};

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";
const BOOK_REVIEW_POST_TAG: &str = "Books";

async fn content_to_omni_post(state: &impl State, content: Content) -> Result<OmniPost> {
    if (content
        .tags()
        .contains(&Tag::from_string(MOVIE_REVIEW_POST_TAG)))
    {
        return match state
            .movie_service()
            .movie_review_from_content(state, &content)
            .await
        {
            Ok(review) => Ok(review.into()),
            Err(e) => {
                warn!(
                    "Could not create movie review from post with slug: {} {:?}",
                    content.slug(),
                    e,
                );
                Ok(content.into())
            }
        };
    }

    if (content
        .tags()
        .contains(&Tag::from_string(TV_SHOW_REVIEW_POST_TAG)))
    {
        return match state
            .tv_shows_service()
            .tv_show_review_from_content(state, &content)
            .await
        {
            Ok(review) => Ok(review.into()),
            Err(e) => {
                warn!(
                    "Could not create movie review from post with slug: {} {:?}",
                    content.slug(),
                    e,
                );
                Ok(content.into())
            }
        };
    }

    if content
        .tags()
        .contains(&Tag::from_string(BOOK_REVIEW_POST_TAG))
    {
        return match state
            .book_service()
            .book_review_from_content(state, &content)
            .await
        {
            Ok(review) => Ok(review.into()),
            Err(e) => {
                warn!(
                    "Could not create book review from post with slug: {} {:?}",
                    content.slug(),
                    e,
                );
                Ok(content.into())
                // return Err(e)
            }
        };
    }

    Ok(content.into())
}

async fn content_posts_to_omni_post(
    state: &impl State,
    content: Vec<Content>,
) -> Result<Vec<OmniPost>> {
    let mut posts = Vec::new();

    for c in content {
        posts.push(content_to_omni_post(state, c).await?);
    }

    Ok(posts)
}

async fn get_blog_post_content(state: &impl State) -> Result<Vec<Content>> {
    let blog_posts = state
        .blog_posts_repo()
        .find_all_by_date()
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Content>>();

    Ok(blog_posts)
}

async fn get_micro_post_content(state: &impl State) -> Result<Vec<Content>> {
    let micro_posts = state
        .micro_posts_repo()
        .find_all()
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Content>>();

    Ok(micro_posts)
}

async fn get_mastodon_post_content(state: &impl State) -> Result<Vec<Content>> {
    let mastodon_posts = state
        .mastodon_posts_repo()
        .find_all_by_date()
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Content>>();

    Ok(mastodon_posts)
}

async fn get_album_content(state: &impl State) -> Result<Vec<Content>> {
    let albums = state
        .albums_repo()
        .find_all_by_date()
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Content>>();

    Ok(albums)
}

async fn get_album_photos_content(state: &impl State) -> Result<Vec<Content>> {
    let photos = state
        .albums_repo()
        .find_all_by_date()
        .await?
        .into_iter()
        .map(|album| {
            album
                .photos
                .iter()
                .map(|photo| (album.clone(), photo.clone()).into())
                .collect::<Vec<Content>>()
        })
        .flatten()
        .collect::<Vec<Content>>();

    Ok(photos)
}

async fn get_steam_achievement_unlocked_content(state: &impl State) -> Result<Vec<OmniPost>> {
    let mut posts = vec![];

    let games = state.steam_games_repo().find_all().await?;

    for game in games {
        let achievements = state
            .steam_achievements_repo()
            .find_all_unlocked_by_unlocked_date(game.id)
            .await?
            .into_iter()
            .map(|a| (game.clone(), a).into())
            .collect::<Vec<OmniPost>>();

        posts.extend(achievements);
    }

    Ok(posts)
}

async fn update_omni_post_content(state: &impl State) -> Result<()> {
    let (blog_posts, micro_posts, mastodon_posts, album_photos, albums) = try_join!(
        get_blog_post_content(state),
        get_micro_post_content(state),
        get_mastodon_post_content(state),
        get_album_photos_content(state),
        get_album_content(state),
    )?;

    let mut content = Vec::new();

    content.extend(blog_posts);
    content.extend(micro_posts);
    content.extend(mastodon_posts);
    content.extend(album_photos);
    content.extend(albums);

    let (steam_achievements_unlocked, converted_posts) = try_join!(
        get_steam_achievement_unlocked_content(state),
        content_posts_to_omni_post(state, content)
    )?;

    let mut omni_posts = Vec::new();
    omni_posts.extend(steam_achievements_unlocked);
    omni_posts.extend(converted_posts);

    state.omni_post_repo().commit(state, omni_posts).await?;

    Ok(())
}

pub async fn update_derived_data_command(state: &impl State) -> Result<()> {
    try_join!(update_omni_post_content(state))?;

    Ok(())
}
