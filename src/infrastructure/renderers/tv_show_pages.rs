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
        repositories::TvShowReviewsRepo,
        services::{MovieService, PageRenderingService, TvShowsService},
        state::State,
    },
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const TV_TAG: &str = "TV";

pub async fn render_tv_show_pages(state: &impl State) -> Result<()> {
    let reviews_by_id = state
        .tv_show_reviews_repo()
        .find_all_grouped_by_tv_show_id()
        .await?;

    render_tv_show_list_page(state, &reviews_by_id).await?;

    for reviews in reviews_by_id.values() {
        if let Some(tv_show) = reviews.first().map(|r| r.tv_show.clone()) {
            render_tv_show_page(state, &tv_show, reviews).await?;
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
pub struct TvShowListTemplate {
    page: Page,
    tv_shows: Vec<AverageReviewForTvShow>,
}

async fn render_tv_show_list_page(
    state: &impl State,
    reviews_by_id: &HashMap<TvShowId, Vec<TvShowReview>>,
) -> Result<()> {
    let mut tv_shows: Vec<AverageReviewForTvShow> = reviews_by_id
        .iter()
        .filter_map(|(id, reviews)| {
            let total_scores = reviews
                .iter()
                .map(|r| r.scores.clone())
                .flatten()
                .collect::<Vec<u8>>();

            let average_score: f32 =
                total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

            match reviews.iter().max_by_key(|r| r.post.date()) {
                Some(most_recent_review) => Some(AverageReviewForTvShow {
                    tv_show: most_recent_review.tv_show.clone(),
                    average_score,
                    most_recent_review: most_recent_review.clone(),
                }),
                None => {
                    error!("No reviews found for tv show with id: {:?}", id);
                    return None;
                }
            }
        })
        .collect();

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

    let template = TvShowListTemplate { page, tv_shows };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await
}

#[derive(Template)]
#[template(path = "interests/tv/tv_show.html")]
pub struct TvShowPageTemplate {
    page: Page,
    tv_show: TvShow,
    average_score: f32,
    posts: Vec<OmniPost>,
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

    let posts = reviews
        .iter()
        .map(|r| r.post.clone())
        .collect::<Vec<OmniPost>>();

    let page = Page::new(tv_show.slug(), Some(&tv_show.title), None);

    let template = TvShowPageTemplate {
        page,
        tv_show: tv_show.clone(),
        average_score,
        posts,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await
}
