use std::collections::HashMap;

use askama::Template;
use tracing::{error, warn};

use crate::{
    domain::{
        models::{
            book::{Book, BookID, BookReview},
            media::Media,
            movie::{Movie, MovieId, MovieReview},
            omni_post::OmniPost,
            page::Page,
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
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_book_pages(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts(state, OmniPostFilterFlags::BOOK_REVIEW).await?;

    let mut reviews_by_id = HashMap::new();
    for post in posts {
        match post {
            OmniPost::BookReview(post) => {
                let entry = reviews_by_id.entry(post.book.id).or_insert_with(Vec::new);
                entry.push(post);
            }
            _ => {}
        }
    }
    render_book_list_page(state, &reviews_by_id).await?;

    for reviews in reviews_by_id.values() {
        if let Some(tv_show) = reviews.first().map(|r| r.book.clone()) {
            render_tv_show_page(state, &tv_show, reviews).await?;
        }
    }

    Ok(())
}

struct AverageReviewForBook {
    book: Book,
    average_score: f32,
    most_recent_review: BookReview,
}

#[derive(Template)]
#[template(path = "interests/books/book_list.html")]
pub struct BookListTemplate {
    page: Page,
    books: Vec<AverageReviewForBook>,
}

async fn render_book_list_page(
    state: &impl State,
    reviews_by_id: &HashMap<BookID, Vec<BookReview>>,
) -> Result<()> {
    let mut books: Vec<AverageReviewForBook> = reviews_by_id
        .iter()
        .filter_map(|(id, reviews)| {
            let total_scores = reviews.iter().map(|r| r.score).collect::<Vec<u8>>();

            let average_score: f32 =
                total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

            match reviews.iter().max_by_key(|r| r.source_content.date()) {
                Some(most_recent_review) => Some(AverageReviewForBook {
                    book: most_recent_review.book.clone(),
                    average_score,
                    most_recent_review: most_recent_review.clone(),
                }),
                None => {
                    error!("No reviews found for book with id: {:?}", id);
                    return None;
                }
            }
        })
        .collect();

    books.sort_by(|a, b| {
        b.most_recent_review
            .source_content
            .date()
            .cmp(&a.most_recent_review.source_content.date())
    });

    let page = Page::new(
        Slug::new("/interests/books"),
        Some("Books"),
        Some("Books I've Read".to_string()),
    );

    let updated_at = books
        .first()
        .map(|r| r.most_recent_review.source_content.date().clone());

    let template = BookListTemplate { page, books };

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
#[template(path = "interests/books/book.html")]
pub struct BookPageTemplate {
    page: Page,
    book: Book,
    average_score: f32,
    posts: Vec<OmniPost>,
}

async fn render_tv_show_page(
    state: &impl State,
    book: &Book,
    reviews: &[BookReview],
) -> Result<()> {
    let total_scores = reviews
        .iter()
        .map(|r| r.score)
        .collect::<Vec<u8>>();

    let average_score: f32 = total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

    let mut posts = reviews
        .iter()
        .map(|r| r.source_content.clone().into())
        .collect::<Vec<OmniPost>>();

    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    let page = Page::new(book.slug(), Some(&book.title), None);

    let template = BookPageTemplate {
        page,
        book: book.clone(),
        average_score,
        posts,
    };

    let most_recent_review = reviews.iter().max_by_key(|r| r.source_content.date());

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            most_recent_review.map(|r| r.source_content.date()),
        )
        .await
}
