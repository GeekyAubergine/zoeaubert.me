use std::collections::HashMap;

use askama::Template;
use tracing::{error, warn};

use crate::{
    domain::models::{
        book::{Book, BookID, BookReview},
        media::Media,
        movie::{Movie, MovieId, MovieReview},
        page::Page,
        post::Post,
        post::PostFilter,
        slug::Slug,
        tag::Tag,
        tv_show::{TvShow, TvShowId, TvShowReview},
    },
    prelude::*,
    renderers::RendererContext,
};

use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

pub async fn render_book_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::BOOK_REVIEW);

    let mut reviews_by_id = HashMap::new();
    for post in posts {
        match post {
            Post::BookReview(post) => {
                let entry = reviews_by_id.entry(post.book.id).or_insert_with(Vec::new);
                entry.push(post);
            }
            _ => {}
        }
    }
    render_book_list_page(context, &reviews_by_id).await?;

    for reviews in reviews_by_id.values() {
        if let Some(tv_show) = reviews.first().map(|r| r.book.clone()) {
            render_tv_show_page(context, &tv_show, &reviews).await?;
        }
    }

    Ok(())
}

struct AverageReviewForBook<'b> {
    book: &'b Book,
    average_score: f32,
    most_recent_review: &'b BookReview,
}

#[derive(Template)]
#[template(path = "interests/books/book_list.html")]
pub struct BookListTemplate<'t> {
    page: Page,
    books: Vec<AverageReviewForBook<'t>>,
}

async fn render_book_list_page(
    context: &RendererContext,
    reviews_by_id: &HashMap<BookID, Vec<&BookReview>>,
) -> Result<()> {
    let mut books: Vec<AverageReviewForBook> = reviews_by_id
        .iter()
        .filter_map(|(id, reviews)| {
            let total_scores = reviews.iter().map(|r| r.score).collect::<Vec<u8>>();

            let average_score: f32 =
                total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

            match reviews.iter().max_by_key(|r| r.source_content.date()) {
                Some(most_recent_review) => Some(AverageReviewForBook {
                    book: &most_recent_review.book,
                    average_score,
                    most_recent_review,
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

    context
        .renderer
        .render_page(&template.page.slug, &template, updated_at)
        .await
}

#[derive(Template)]
#[template(path = "interests/books/book.html")]
pub struct BookPageTemplate<'t> {
    page: Page,
    book: &'t Book,
    average_score: f32,
    posts: Vec<Post>,
}

async fn render_tv_show_page(
    context: &RendererContext,
    book: &Book,
    reviews: &Vec<&BookReview>,
) -> Result<()> {
    let total_scores = reviews.iter().map(|r| r.score).collect::<Vec<u8>>();

    let average_score: f32 = total_scores.iter().sum::<u8>() as f32 / total_scores.len() as f32;

    let mut posts = reviews
        .iter()
        .map(|r| r.source_content.clone().into())
        .collect::<Vec<Post>>();

    posts.sort_by(|a, b| b.date().cmp(&a.date()));

    let page = Page::new(book.slug(), Some(&book.title), None);

    let template = BookPageTemplate {
        page,
        book,
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
