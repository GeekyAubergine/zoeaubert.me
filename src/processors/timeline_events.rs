use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tracing::{error, instrument};

use crate::{
    domain::models::{
        blog_post::BlogPost,
        games::{Game, Games},
        mastodon_post::{MastodonPost, MastodonPosts},
        micro_post::MicroPost,
        review::{
            book_review::BookReview, movie_review::MovieReview, review_source::ReviewSource,
            tv_show_review::TvShowReview,
        },
        tag::Tag,
        timeline_event::{
            TimelineEvent, TimelineEventGameAchievementUnlock, TimelineEventPost,
            TimelineEventReview, TimelineEvents,
        },
    },
    services::ServiceContext,
};

const MOVIE_REVIEW_POST_TAG: &str = "Movies";
const TV_SHOW_REVIEW_POST_TAG: &str = "TV";
const BOOK_REVIEW_POST_TAG: &str = "Books";

#[instrument(skip_all, fields(source.slug=%source.slug()))]
fn process_review_source(ctx: &ServiceContext, source: ReviewSource) -> TimelineEvent {
    if (source
        .tags()
        .contains(&Tag::from_string(BOOK_REVIEW_POST_TAG)))
    {
        if let Ok(review) = BookReview::from_content(&source.content()) {
            let book = ctx
                .books
                .find_book(ctx, &review.title, &review.author, source.tags());

            return match book {
                Ok(Some(book)) => TimelineEvent::Review(TimelineEventReview::BookReview {
                    review,
                    book,
                    source,
                }),
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
            let movie = ctx.movies.find_movie(ctx, &review.title, review.year);

            return match movie {
                Ok(Some(movie)) => TimelineEvent::Review(TimelineEventReview::MovieReview {
                    review,
                    movie,
                    source,
                }),
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

    if (source
        .tags()
        .contains(&Tag::from_string(TV_SHOW_REVIEW_POST_TAG)))
    {
        if let Ok(review) = TvShowReview::from_content(&source.content()) {
            let tv_show = ctx.tv_shows.find_tv_show(ctx, &review.title);

            return match tv_show {
                Ok(Some(tv_show)) => TimelineEvent::Review(TimelineEventReview::TvShowReview {
                    review,
                    tv_show,
                    source,
                }),
                Ok(None) => source.into(),
                Err(e) => {
                    let slug = source.slug();
                    let title = review.title;
                    error!("Unable to process movie post [{slug}] [{title}]");
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

fn extract_events_from_micro_posts(
    ctx: &ServiceContext,
    micro_posts: Vec<MicroPost>,
) -> Vec<TimelineEvent> {
    micro_posts
        .into_par_iter()
        .map(|post| process_review_source(ctx, ReviewSource::MicroPost(post)))
        .collect()
}

fn extract_events_from_mastodon(
    ctx: &ServiceContext,
    mastodon_posts: MastodonPosts,
) -> Vec<TimelineEvent> {
    mastodon_posts
        .posts()
        .into_par_iter()
        .map(|post| process_review_source(ctx, ReviewSource::MastodonPost(post.clone())))
        .collect()
}

fn extract_events_from_games<'l>(ctx: &ServiceContext, games: &Games) -> Vec<TimelineEvent> {
    let mut events = vec![];

    for game in games.find_all() {
        match game {
            Game::Steam(game) => {
                let achievement_events: Vec<TimelineEvent> =
                    game.unlocked_achievements.par_iter().map(|(_, achievement)| {
                        TimelineEvent::GameAchievementUnlock(
                            TimelineEventGameAchievementUnlock::SteamAchievementUnlocked {
                                game: game.game.clone(),
                                achievement: achievement.clone(),
                            },
                        )
                    }).collect();

                events.extend(achievement_events);
            }
        }
    }

    events
}

pub fn process_timeline_events(
    ctx: &ServiceContext,
    blog_posts: Vec<BlogPost>,
    micro_posts: Vec<MicroPost>,
    mastodon_posts: MastodonPosts,
    games: &Games,
) -> TimelineEvents {
    let mut events: Vec<TimelineEvent> = vec![];

    events.extend(extract_events_from_blog_posts(ctx, blog_posts));
    events.extend(extract_events_from_micro_posts(ctx, micro_posts));
    events.extend(extract_events_from_mastodon(ctx, mastodon_posts));
    events.extend(extract_events_from_games(ctx, games));

    TimelineEvents::from_events(events)
}
