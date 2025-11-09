use image::imageops::FilterType::Lanczos3;
use tracing::{error, instrument, span, warn, Level};

use crate::{
    domain::models::{
        albums::{album::Album, Albums},
        blog_post::BlogPost,
        book::BookReview,
        games::{Game, Games},
        mastodon_post::{MastodonPost, MastodonPosts},
        micro_post::MicroPost,
        post::Post,
        source_post::SourcePost,
        tag::Tag,
    },
    prelude::*,
    services::ServiceContext,
    utils::parse_content_into_book_review::parse_content_into_book_review,
};

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";
const BOOK_REVIEW_POST_TAG: &str = "Books";

pub fn extract_posts_from_blogs(blog_posts: Vec<BlogPost>) -> Vec<SourcePost> {
    blog_posts.into_iter().map(SourcePost::BlogPost).collect()
}

pub fn extract_posts_from_micro_posts(micro_posts: Vec<MicroPost>) -> Vec<SourcePost> {
    micro_posts.into_iter().map(SourcePost::MicroPost).collect()
}

pub fn extract_posts_from_mastodon_posts(mastodon_posts: &MastodonPosts) -> Vec<SourcePost> {
    mastodon_posts
        .posts()
        .into_iter()
        .map(|post| SourcePost::MastodonPost(post.clone()))
        .collect()
}

#[instrument(skip_all, fields(post.slug=%post.slug()))]
pub async fn process_source_post(ctx: &ServiceContext, post: SourcePost) -> Post {
    if (post
        .tags()
        .contains(&Tag::from_string(BOOK_REVIEW_POST_TAG)))
    {
        if let Ok(review) = parse_content_into_book_review(&post) {
            let book = ctx
                .books
                .find_book(ctx, &review.title, &review.author, &post.tags())
                .await;

            return match book {
                Ok(Some(book)) => Post::BookReview(BookReview {
                    book,
                    score: review.score,
                    review: review.review,
                    source_content: post.clone(),
                }),
                Ok(None) => post.into(),
                Err(e) => {
                    let slug = post.slug();
                    let title = review.title;
                    error!("Unable to process book post [{slug}] [{title}]");
                    post.into()
                }
            };
        }
    }

    post.into()
}

pub async fn process_source_posts(
    ctx: &ServiceContext,
    blog_posts: Vec<BlogPost>,
    micro_posts: Vec<MicroPost>,
    mastodon_posts: &MastodonPosts,
) -> Result<Vec<Post>> {
    let mut source_posts = vec![];

    source_posts.extend(extract_posts_from_blogs(blog_posts));
    source_posts.extend(extract_posts_from_micro_posts(micro_posts));
    source_posts.extend(extract_posts_from_mastodon_posts(mastodon_posts));

    let mut posts = vec![];

    for post in source_posts {
        posts.push(process_source_post(ctx, post).await);
    }

    Ok(posts)
}
