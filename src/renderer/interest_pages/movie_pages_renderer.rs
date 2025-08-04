use std::collections::HashMap;

use askama::Template;
use tracing::{error, warn};

use crate::{
    domain::{
        models::{
            post::PostFilter,
            media::Media,
            movie::{Movie, MovieId, MovieReview},
            post::Post,
            page::Page,
            slug::Slug,
            tag::Tag,
        },
    },
    renderers::RendererContext,
    prelude::*,
};

use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

const MOVIE_TAG: &str = "Movies";

pub async fn render_movie_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::MOVIE_REVIEW);

    let mut reviews_by_id: HashMap<MovieId, Vec<&MovieReview>> = HashMap::new();
    for post in posts {
        match post {
            Post::MovieReview(post) => {
                let entry = reviews_by_id.entry(post.movie.id).or_insert_with(Vec::new);
                entry.push(post);
            }
            _ => {}
        }
    }

    render_movies_list_page(context, &reviews_by_id).await?;

    for reviews in reviews_by_id.into_values() {
        if let Some(movie) = reviews.first().map(|r| r.movie.clone()) {
            render_movie_page(context, &movie, &reviews).await?;
        }
    }

    Ok(())
}

struct AverageReviewForMovie<'m> {
    movie: &'m Movie,
    average_score: f32,
    most_recent_review: &'m MovieReview,
}

#[derive(Template)]
#[template(path = "interests/movies/movies_list.html")]
pub struct MovieListTemplate<'t> {
    page: Page,
    movies: Vec<AverageReviewForMovie<'t>>,
}

async fn render_movies_list_page(
    context: &RendererContext,
    reviews_by_id: &HashMap<MovieId, Vec<&MovieReview>>,
) -> Result<()> {
    let mut movies: Vec<AverageReviewForMovie> = reviews_by_id
        .iter()
        .filter_map(|(id, reviews)| {
            let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
            let average_score = (total_score as f32 / reviews.len() as f32);

            match reviews.iter().max_by_key(|r| r.source_content.date()) {
                Some(most_recent_review) => Some(AverageReviewForMovie {
                    movie: &most_recent_review.movie,
                    average_score,
                    most_recent_review,
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
            .source_content
            .date()
            .cmp(&a.most_recent_review.source_content.date())
    });

    let page = Page::new(
        Slug::new("/interests/movies"),
        Some("Movies"),
        Some("Movies I've watched".to_string()),
    );

    let updated_at = movies
        .first()
        .map(|r| r.most_recent_review.source_content.date().clone());

    let template = MovieListTemplate { page, movies };

    context
        .renderer
        .render_page(&template.page.slug, &template, updated_at)
        .await
}

#[derive(Template)]
#[template(path = "interests/movies/movie.html")]
pub struct MoviePageTemplate<'t> {
    page: Page,
    movie: &'t Movie,
    average_score: f32,
    posts: Vec<Post>,
}

async fn render_movie_page(
    context: &RendererContext,
    movie: &Movie,
    reviews: &Vec<&MovieReview>,
) -> Result<()> {
    let average_score: f32 = {
        let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
        (total_score as f32 / reviews.len() as f32)
    };

    let mut posts = reviews
        .iter()
        .map(|r| r.source_content.clone().into())
        .collect::<Vec<Post>>();

    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    let page = Page::new(movie.slug(), Some(&movie.title), None);

    let template = MoviePageTemplate {
        page,
        movie,
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
