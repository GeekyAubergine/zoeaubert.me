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
        },
        queries::omni_post_queries::find_all_omni_posts_by_tag,
        repositories::MovieReviewsRepo,
        services::{MovieService, PageRenderingService},
        state::State,
    },
    prelude::*,
};

use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;

const MOVIE_TAG: &str = "Movies";

pub async fn render_movie_pages(state: &impl State) -> Result<()> {
    let reviews_by_id = state
        .movie_reviews_repo()
        .find_all_grouped_by_movie_id()
        .await?;

    render_movies_list_page(state, &reviews_by_id).await?;

    for reviews in reviews_by_id.into_values() {
        if let Some(movie) = reviews.first().map(|r| r.movie.clone()) {
            render_movie_page(state, movie, reviews).await?;
        }
    }

    Ok(())
}

struct AverageReviewForMovie {
    movie: Movie,
    average_score: f32,
    most_recent_review: MovieReview,
}

#[derive(Template)]
#[template(path = "interests/movies/movies_list.html")]
pub struct MovieListTemplate {
    page: Page,
    movies: Vec<AverageReviewForMovie>,
}

async fn render_movies_list_page(
    state: &impl State,
    reviews_by_id: &HashMap<MovieId, Vec<MovieReview>>,
) -> Result<()> {
    let mut movies: Vec<AverageReviewForMovie> = reviews_by_id
        .iter()
        .filter_map(|(id, reviews)| {
            let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
            let average_score = (total_score as f32 / reviews.len() as f32);

            match reviews.iter().max_by_key(|r| r.post.date()) {
                Some(most_recent_review) => Some(AverageReviewForMovie {
                    movie: most_recent_review.movie.clone(),
                    average_score,
                    most_recent_review: most_recent_review.clone(),
                }),
                None => {
                    error!("No reviews found for movie with id: {:?}", id);
                    return None;
                }
            }
        })
        .collect();

    movies.sort_by(|a, b| {
        b.most_recent_review
            .post
            .date()
            .cmp(&a.most_recent_review.post.date())
    });

    let page = Page::new(
        Slug::new("/interests/movies"),
        Some("Movies"),
        Some("Movies I've watched"),
    );

    let updated_at = movies
        .first()
        .map(|r| r.most_recent_review.post.date().clone());

    let template = MovieListTemplate { page, movies };

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            updated_at.as_ref(),
        )
        .await
}

#[derive(Template)]
#[template(path = "interests/movies/movie.html")]
pub struct MoviePageTemplate {
    page: Page,
    movie: Movie,
    average_score: f32,
    posts: Vec<OmniPost>,
}

async fn render_movie_page(
    state: &impl State,
    movie: Movie,
    reviews: Vec<MovieReview>,
) -> Result<()> {
    let average_score: f32 = {
        let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
        (total_score as f32 / reviews.len() as f32)
    };

    let mut posts = reviews
        .iter()
        .map(|r| r.post.clone())
        .collect::<Vec<OmniPost>>();

    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    let page = Page::new(movie.slug(), Some(&movie.title), None);

    let template = MoviePageTemplate {
        page,
        movie: movie.clone(),
        average_score,
        posts,
    };

    let most_recent_review = reviews.iter().max_by_key(|r| r.post.date());

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            most_recent_review.map(|r| r.post.date()),
        )
        .await
}
