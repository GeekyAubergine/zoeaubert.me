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
        services::MovieService,
        state::State,
    },
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::render_page_with_template;

const MOVIE_TAG: &str = "Movies";

pub async fn render_movie_pages(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts_by_tag(state, &Tag::from_string(MOVIE_TAG)).await?;

    let mut reviews = Vec::new();

    for post in posts {
        match state
            .movie_service()
            .movie_review_from_omni_post(state, &post)
            .await
        {
            Ok(review) => reviews.push(review),
            Err(_) => {
                warn!(
                    "Could not create movie review from post with slug: {}",
                    post.slug()
                );
            }
        }
    }

    let movies_by_id: HashMap<MovieId, Movie> =
        reviews.iter().fold(HashMap::new(), |mut acc, review| {
            acc.insert(review.movie.id, review.movie.clone());
            acc
        });

    let reviews_by_id: HashMap<MovieId, Vec<MovieReview>> =
        reviews.iter().fold(HashMap::new(), |mut acc, review| {
            acc.entry(review.movie.id)
                .or_insert_with(Vec::new)
                .push(review.clone());
            acc
        });

    render_movies_list_page(state, &reviews, &movies_by_id, &reviews_by_id).await?;

    for movie in movies_by_id.values() {
        match reviews_by_id.get(&movie.id) {
            Some(reviews) => render_movie_page(state, &movie, &reviews).await?,
            None => {}
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
pub struct MovieListTempalte<'t> {
    page: &'t Page<'t>,
    movies: Vec<AverageReviewForMovie>,
}

async fn render_movies_list_page(
    state: &impl State,
    reviews: &[MovieReview],
    movies_by_id: &HashMap<MovieId, Movie>,
    reviews_by_id: &HashMap<MovieId, Vec<MovieReview>>,
) -> Result<()> {
    let average_score_by_id: HashMap<MovieId, f32> = reviews_by_id
        .iter()
        .map(|(id, reviews)| {
            let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
            let average_score = (total_score as f32 / reviews.len() as f32);
            (*id, average_score)
        })
        .collect();

    let most_recent_review_by_id: HashMap<MovieId, MovieReview> = reviews_by_id
        .iter()
        .map(|(id, reviews)| {
            let most_recent_review = reviews.iter().max_by_key(|r| r.post.date()).unwrap();
            (*id, most_recent_review.clone())
        })
        .collect();

    let mut movies = movies_by_id
        .iter()
        .map(|(id, movie)| {
            let average_score = average_score_by_id[id];
            let most_recent_review = most_recent_review_by_id[id].clone();
            AverageReviewForMovie {
                movie: movie.clone(),
                average_score,
                most_recent_review,
            }
        })
        .collect::<Vec<_>>();

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

    let template = MovieListTempalte {
        page: &page,
        movies,
    };

    render_page_with_template(state, &page, template).await
}

#[derive(Template)]
#[template(path = "interests/movies/movie.html")]
pub struct MoviePageTempalte<'t> {
    page: &'t Page<'t>,
    movie: Movie,
    average_score: f32,
    posts: Vec<&'t OmniPost>,
}

async fn render_movie_page(
    state: &impl State,
    movie: &Movie,
    reviews: &[MovieReview],
) -> Result<()> {
    let average_score: f32 = {
        let total_score: u16 = reviews.iter().map(|r| r.score as u16).sum();
        (total_score as f32 / reviews.len() as f32)
    };

    let posts = reviews.iter().map(|r| &r.post).collect::<Vec<&OmniPost>>();

    let page = Page::new(movie.slug(), Some(&movie.title), None);

    let template = MoviePageTempalte {
        page: &page,
        movie: movie.clone(),
        average_score,
        posts,
    };

    render_page_with_template(state, &page, template).await
}
