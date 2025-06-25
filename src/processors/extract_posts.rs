use crate::{
    domain::models::{
        albums::{album::Album, Albums},
        blog_post::BlogPost,
        games::{Game, Games},
        mastodon_post::{MastodonPost, MastodonPosts},
        micro_post::MicroPost,
        post::Post,
    },
    prelude::*,
    services::ServiceContext,
};

pub fn extract_posts_from_blogs(blog_posts: Vec<BlogPost>) -> Vec<Post> {
    blog_posts.into_iter().map(Post::BlogPost).collect()
}

pub fn extract_posts_from_micro_posts(micro_posts: Vec<MicroPost>) -> Vec<Post> {
    micro_posts.into_iter().map(Post::MicroPost).collect()
}

pub fn extract_posts_from_mastodon_posts(mastodon_posts: &MastodonPosts) -> Vec<Post> {
    mastodon_posts
        .posts()
        .into_iter()
        .map(|post| Post::MastodonPost(post.clone()))
        .collect()
}

pub fn extract_posts_from_albums(albums: &Albums) -> Vec<Post> {
    albums
        .find_all_by_date()
        .into_iter()
        .flat_map(|album| {
            let mut posts = vec![Post::Album(album.clone())];

            posts.extend(
                album
                    .photos
                    .iter()
                    .map(|photo| Post::AlbumPhoto {
                        album: album.clone(),
                        photo: photo.clone(),
                    })
                    .collect::<Vec<Post>>(),
            );

            posts
        })
        .collect()
}

pub fn extra_post_from_games(games: &Games) -> Vec<Post> {
    games
        .find_all()
        .iter()
        .flat_map(|game| match game {
            Game::Steam(steam_game) => steam_game
                .find_all_unlocked_by_unlocked_date()
                .into_iter()
                .map(|achievement| Post::SteamAcheivementUnlocked {
                    game: steam_game.game.clone(),
                    achievement: achievement.clone(),
                }),
        })
        .collect()
}

// TODO extract reviews and stuff

pub async fn extract_posts(
    ctx: &ServiceContext,
    blog_posts: Vec<BlogPost>,
    micro_posts: Vec<MicroPost>,
    mastodon_posts: &MastodonPosts,
    albums: &Albums,
    games: &Games,
) -> Result<Vec<Post>> {
    let mut posts = vec![];

    posts.extend(extract_posts_from_blogs(blog_posts));
    posts.extend(extract_posts_from_micro_posts(micro_posts));
    posts.extend(extract_posts_from_mastodon_posts(mastodon_posts));
    posts.extend(extract_posts_from_albums(albums));
    posts.extend(extra_post_from_games(games));

    Ok(posts)
}
