use tracing::{error, instrument};

use crate::{
    domain::models::{
        blog_post::BlogPost,
        mastodon_post::{MastodonPost, MastodonPosts},
        micro_post::MicroPost,
        review::{book_review::BookReview, movie_review::MovieReview, review_source::ReviewSource},
        tag::Tag,
        timeline_event::{TimelineEvent, TimelineEventPost, TimelineEvents},
    },
    services::ServiceContext,
};

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";
const BOOK_REVIEW_POST_TAG: &str = "Books";

#[instrument(skip_all, fields(source.slug=%source.slug()))]
async fn process_review_source(ctx: &ServiceContext, source: ReviewSource) -> TimelineEvent {
    if (source
        .tags()
        .contains(&Tag::from_string(BOOK_REVIEW_POST_TAG)))
    {
        if let Ok(review) = BookReview::from_content(&source.content()) {
            let book = ctx
                .books
                .find_book(ctx, &review.title, &review.author, source.tags())
                .await;

            return match book {
                Ok(Some(book)) => TimelineEvent::BookReview {
                    review,
                    book,
                    source,
                },
                Ok(None) => source.into(),
                Err(e) => {
                    let slug = source.slug();
                    let title = review.title;
                    error!("Unable to process book post [{slug}] [{title}]");
                    source.into()
                }
            };
        }
    }

    if (source
        .tags()
        .contains(&Tag::from_string(MOVIE_REVIEW_POST_TAG)))
    {
        if let Ok(review) = MovieReview::from_content(&source.content()) {
            let movie = ctx
                .movies
                .find_movie(ctx, &review.title, review.year)
                .await;

            return match movie {
                Ok(Some(movie)) => TimelineEvent::MovieReview {
                    review,
                    movie,
                    source,
                },
                Ok(None) => source.into(),
                Err(e) => {
                    let slug = source.slug();
                    let title = review.title;
                    let year = review.year;
                    error!("Unable to process movie post [{slug}] [{title} - {year}]");
                    source.into()
                }
            };
        }
    }

    source.into()
}

fn extract_events_from_blog_posts(
    ctx: &ServiceContext,
    blog_posts: Vec<BlogPost>,
) -> impl Iterator<Item = TimelineEvent> {
    blog_posts
        .into_iter()
        .map(|post| TimelineEvent::Post(TimelineEventPost::BlogPost(post)))
}

async fn extract_events_from_micro_posts(
    ctx: &ServiceContext,
    micro_posts: Vec<MicroPost>,
) -> Vec<TimelineEvent> {
    let mut events = vec![];

    for post in micro_posts {
        events.push(process_review_source(ctx, ReviewSource::MicroPost(post)).await);
    }

    events
}

async fn extract_events_from_mastodon(
    ctx: &ServiceContext,
    mastodon_posts: MastodonPosts,
) -> Vec<TimelineEvent> {
    let mut events = vec![];

    for post in mastodon_posts.posts() {
        events.push(process_review_source(ctx, ReviewSource::MastodonPost(post.clone())).await);
    }

    events
}

pub async fn process_timeline_events(
    ctx: &ServiceContext,
    blog_posts: Vec<BlogPost>,
    micro_posts: Vec<MicroPost>,
    mastodon_posts: MastodonPosts,
) -> TimelineEvents {
    let mut events: Vec<TimelineEvent> = vec![];

    events.extend(extract_events_from_blog_posts(ctx, blog_posts));
    events.extend(extract_events_from_micro_posts(ctx, micro_posts).await);
    events.extend(extract_events_from_mastodon(ctx, mastodon_posts).await);

    TimelineEvents::from_events(events)
}
