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
            post::PostFilter,
            slug::Slug,
            tag::Tag,
            tv_show::{TvShow, TvShowId, TvShowReview},
        },
        queries::omni_post_queries::{
            find_all_omni_posts, find_all_omni_posts_by_tag, OmniPostFilterFlags,
        },
        repositories::TvShowReviewsRepo,
        services::{MovieService, PageRenderingService, TvShowsService},
        state::State,
    },
    infrastructure::renderers::RendererContext,
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_tv_show_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::TV_SHOW_REVIEW);

    let mut reviews_by_id: HashMap<TvShowId, Vec<&TvShowReview>> = HashMap::new();
    for post in posts {
        match post {
            OmniPost::TvShowReview(post) => {
                let entry = reviews_by_id
                    .entry(post.tv_show.id)
                    .or_insert_with(Vec::new);
                entry.push(post);
            }
            _ => {}
        }
    }
    render_tv_show_list_page(context, &reviews_by_id).await?;

    for reviews in reviews_by_id.values() {
        if let Some(tv_show) = reviews.first().map(|r| r.tv_show.clone()) {
            render_tv_show_page(context, &tv_show, reviews).await?;
        }
    }

    Ok(())
}

struct AverageReviewForTvShow<'t> {
    tv_show: &'t TvShow,
    average_score: f32,
    most_recent_review: &'t TvShowReview,
}

#[derive(Template)]
#[template(path = "interests/tv/tv_show_list.html")]
pub struct TvShowListTemplate<'t> {
    page: Page,
    tv_shows: Vec<AverageReviewForTvShow<'t>>,
}

async fn render_tv_show_list_page(
    context: &RendererContext,
    reviews_by_id: &HashMap<TvShowId, Vec<&TvShowReview>>,
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

            match reviews.iter().max_by_key(|r| r.source_content.date()) {
                Some(most_recent_review) => Some(AverageReviewForTvShow {
                    tv_show: &most_recent_review.tv_show,
                    average_score,
                    most_recent_review,
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
            .source_content
            .date()
            .cmp(&a.most_recent_review.source_content.date())
    });

    let page = Page::new(
        Slug::new("/interests/tv"),
        Some("Tv"),
        Some("Tv shows I've watched".to_string()),
    );

    let updated_at = tv_shows
        .first()
        .map(|r| r.most_recent_review.source_content.date().clone());

    let template = TvShowListTemplate { page, tv_shows };

    context
        .renderer
        .render_page(&template.page.slug, &template, updated_at)
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
    context: &RendererContext,
    tv_show: &TvShow,
    reviews: &Vec<&TvShowReview>,
) -> Result<()> {
    let total_scores = reviews
        .iter()
        .map(|r| r.scores.clone())
        .flatten()
        .collect::<Vec<u8>>();

    let average_score: f32 = total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

    let mut posts = reviews
        .iter()
        .map(|r| r.source_content.clone().into())
        .collect::<Vec<OmniPost>>();

    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    let page = Page::new(tv_show.slug(), Some(&tv_show.title), None);

    let template = TvShowPageTemplate {
        page,
        tv_show: tv_show.clone(),
        average_score,
        posts,
    };

    let updated_at = reviews
        .iter()
        .max_by_key(|r| r.source_content.date())
        .map(|r| *r.source_content.date());

    context
        .renderer
        .render_page(&template.page.slug, &template, updated_at)
        .await
}
