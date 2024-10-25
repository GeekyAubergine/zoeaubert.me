use std::collections::HashMap;

use askama::Template;
use tracing::{error, warn};

use crate::{
    domain::{
        models::{
            media::Media,
            movie::{Movie, MovieId, MovieReview},
            omni_post::OmniPost,
            page::Page,
            slug::Slug,
            tag::Tag,
            tv_show::{TvShow, TvShowId, TvShowReview},
        },
        queries::omni_post_queries::find_all_omni_posts_by_tag,
        services::{MovieService, TvShowsService},
        state::State,
    },
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::render_page_with_template;

const TV_TAG: &str = "TV";

pub async fn render_tv_show_pages(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts_by_tag(state, &Tag::from_string(TV_TAG)).await?;

    println!("Found {} posts with tag: {}", posts.len(), TV_TAG);

    let mut reviews = Vec::new();

    for post in posts {
        match state
            .tv_shows_service()
            .tv_show_review_from_omni_post(state, &post)
            .await
        {
            Ok(review) => {
                reviews.push(review);
            }
            Err(e) => {
                warn!(
                    "Could not create tv show review from post with slug: {} {:?}",
                    post.slug(),
                    e,
                );
            }
        }
    }

    let tv_shows_by_id: HashMap<TvShowId, TvShow> =
        reviews.iter().fold(HashMap::new(), |mut acc, review| {
            acc.insert(review.tv_show.id, review.tv_show.clone());
            acc
        });

    let reviews_by_id: HashMap<TvShowId, Vec<TvShowReview>> =
        reviews.iter().fold(HashMap::new(), |mut acc, review| {
            acc.entry(review.tv_show.id)
                .or_insert_with(Vec::new)
                .push(review.clone());
            acc
        });

    render_tv_show_list_page(state, &reviews, &tv_shows_by_id, &reviews_by_id).await?;

    for tv_show in tv_shows_by_id.values() {
        match reviews_by_id.get(&tv_show.id) {
            Some(reviews) => render_tv_show_page(state, &tv_show, &reviews).await?,
            None => {}
        }
    }

    Ok(())
}

struct AverageReviewForTvShow {
    tv_show: TvShow,
    average_score: f32,
    most_recent_review: TvShowReview,
}

#[derive(Template)]
#[template(path = "interests/tv/tv_show_list.html")]
pub struct TvShowListTempalte<'t> {
    page: &'t Page<'t>,
    tv_shows: Vec<AverageReviewForTvShow>,
}

async fn render_tv_show_list_page(
    state: &impl State,
    reviews: &[TvShowReview],
    movies_by_id: &HashMap<TvShowId, TvShow>,
    reviews_by_id: &HashMap<TvShowId, Vec<TvShowReview>>,
) -> Result<()> {
    let average_score_by_id: HashMap<TvShowId, f32> = reviews_by_id
        .iter()
        .map(|(id, reviews)| {
            let total_scores = reviews
                .iter()
                .map(|r| r.scores.clone())
                .flatten()
                .collect::<Vec<u8>>();

            let average_score: f32 =
                total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

            (*id, average_score)
        })
        .collect();

    let most_recent_review_by_id: HashMap<TvShowId, TvShowReview> = reviews_by_id
        .iter()
        .map(|(id, reviews)| {
            let most_recent_review = reviews.iter().max_by_key(|r| r.post.date()).unwrap();
            (*id, most_recent_review.clone())
        })
        .collect();

    let mut tv_shows = movies_by_id
        .iter()
        .map(|(id, tv_show)| {
            let average_score = average_score_by_id[id];
            let most_recent_review = most_recent_review_by_id[id].clone();
            AverageReviewForTvShow {
                tv_show: tv_show.clone(),
                average_score,
                most_recent_review,
            }
        })
        .collect::<Vec<_>>();

    tv_shows.sort_by(|a, b| {
        b.most_recent_review
            .post
            .date()
            .cmp(&a.most_recent_review.post.date())
    });

    let page = Page::new(
        Slug::new("/interests/tv"),
        Some("Tv"),
        Some("Tv shows I've watched"),
    );

    let template = TvShowListTempalte {
        page: &page,
        tv_shows,
    };

    render_page_with_template(state, &page, template).await
}

#[derive(Template)]
#[template(path = "interests/tv/tv_show.html")]
pub struct TvShowPageTempalte<'t> {
    page: &'t Page<'t>,
    tv_show: TvShow,
    average_score: f32,
    posts: Vec<&'t OmniPost>,
}

async fn render_tv_show_page(
    state: &impl State,
    tv_show: &TvShow,
    reviews: &[TvShowReview],
) -> Result<()> {
    let total_scores = reviews
        .iter()
        .map(|r| r.scores.clone())
        .flatten()
        .collect::<Vec<u8>>();

    let average_score: f32 = total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

    let posts = reviews.iter().map(|r| &r.post).collect::<Vec<&OmniPost>>();

    let page = Page::new(tv_show.slug(), Some(&tv_show.title), None);

    let template = TvShowPageTempalte {
        page: &page,
        tv_show: tv_show.clone(),
        average_score,
        posts,
    };

    render_page_with_template(state, &page, template).await
}
