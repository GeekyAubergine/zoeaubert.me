use std::collections::HashMap;

use chrono::{DateTime, Utc};
use once_cell::unsync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::models::mastodon_post::MastodonPost;
use crate::error::BookError;
use crate::prelude::*;

use crate::domain::models::{
    image::Image, media::Media, micro_post::MicroPost, review::review_source::ReviewSource,
    slug::Slug,
};

const REGEX_MICRO: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(.+) by (.+)+\s*(\d)/\d+ - (.+)$").unwrap());

const REGEX_LEGACY_STYLE_AUTHOR_TITLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[(.*)\].*by (.*) 📚").unwrap());

#[derive(Debug, Clone)]
pub struct BookReview {
    pub title: String,
    pub author: String,
    pub score: u8,
    pub review: String,
    pub source: ReviewSource,
}

impl BookReview {
    pub fn from_micropost(post: MicroPost) -> Result<BookReview> {
        parse_markdown_into_book_review(ReviewSource::MicroPost(post.clone()), &post.content)
    }

    pub fn from_mastodon_post(post: MastodonPost) -> Result<BookReview> {
        parse_markdown_into_book_review(ReviewSource::MastodonPost(post.clone()), &post.content())
    }

    pub fn from_review_source(source: ReviewSource) -> Result<BookReview> {
        parse_markdown_into_book_review(source.clone(), source.content())
    }
}

pub struct BookReviews {
    book_reviews: HashMap<Slug, BookReview>,
}

impl BookReviews {
    pub fn insert(&mut self, review: BookReview) {
        self.book_reviews.insert(review.source.slug(), review);
    }

    pub fn all(&self) -> &HashMap<Slug, BookReview> {
        &self.book_reviews
    }
}

fn parse_markdown_into_book_review(source: ReviewSource, content: &str) -> Result<BookReview> {
    if (content.starts_with("Finished") || content.starts_with("I finished")) {
        return parse_legacy_post(source, content);
    }

    if (content.starts_with("<p>")) {
        return parse_mastodon_modern_post(source, content);
    }

    parse_new_style(source, content)
}

fn parse_legacy_post(source: ReviewSource, content: &str) -> Result<BookReview> {
    let content = content.replace("\\n", "\n");

    let lines = content.lines().collect::<Vec<&str>>();

    let lines = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    if let (Some(first_line), Some(second_line)) = (lines.get(0), lines.get(1)) {
        let captures = REGEX_LEGACY_STYLE_AUTHOR_TITLE
            .captures(&content)
            .ok_or(BookError::unable_to_parse_book(content.to_string()))?;

        let second_line_split = second_line.split("/5 - ").collect::<Vec<&str>>();

        if let (Some(title), Some(author), Some(score), Some(review)) = (
            captures.get(1),
            captures.get(2),
            second_line_split.get(0),
            second_line_split.get(1),
        ) {
            let score = score.trim().parse().unwrap();
            return Ok(BookReview {
                title: title.as_str().to_string(),
                author: author.as_str().to_string(),
                score,
                review: review.trim().to_string(),
                source,
            });
        }
    }

    Err(BookError::unable_to_parse_book(content.to_string()))
}

fn parse_mastodon_modern_post(source: ReviewSource, content: &str) -> Result<BookReview> {
    let content = content
        .replace("</p><p>", "\n")
        .replace("<p>", "")
        .replace("</p>", "");

    parse_new_style(source, &content)
}

fn parse_new_style(source: ReviewSource, content: &str) -> Result<BookReview> {
    let content = content.replace("\\n", "\n");

    let lines = content.lines().collect::<Vec<&str>>();

    let lines = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    if let (Some(first_line), Some(second_line)) = (lines.get(0), lines.get(1)) {
        let first_line_split = first_line.split(" by ").collect::<Vec<&str>>();

        let has_review = second_line.contains("/5 - ");

        let split = match has_review {
            true => "/5 - ",
            false => "/5",
        };

        let second_line_split = second_line.split(split).collect::<Vec<&str>>();

        if let (Some(title), Some(author), Some(score), Some(review)) = (
            first_line_split.get(0),
            first_line_split.get(1),
            second_line_split.get(0),
            second_line_split.get(1),
        ) {
            let score = score.trim().parse().unwrap();
            return Ok(BookReview {
                title: title.trim().to_string(),
                author: author.trim().to_string(),
                score,
                review: review.trim().to_string(),
                source,
            });
        }
    }

    Err(BookError::unable_to_parse_book(content.to_string()))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_mastodon_modern_post() {
//         let content = r#"<p>Legion by Dan Abnett</p><p>2/5 - The story is good, but I found it hard to follow in places.</p>"#;

//         let review = parse_markdown_into_book_review(content).unwrap();

//         assert_eq!(review.title, "Legion");
//         assert_eq!(review.author, "Dan Abnett");
//         assert_eq!(review.score, 2);
//         assert_eq!(
//             review.review,
//             "The story is good, but I found it hard to follow in places."
//         );
//     }

//     #[test]
//     fn test_parse_markdown_modern_post() {
//         let content = r#"The First Heretic by Aaron Dembski-Bowden

//         4/5 - Incredible. I'm looking forward to reading this again"#;

//         let review = parse_markdown_into_book_review(content).unwrap();

//         assert_eq!(review.title, "The First Heretic");
//         assert_eq!(review.author, "Aaron Dembski-Bowden");
//         assert_eq!(review.score, 4);
//         assert_eq!(
//             review.review,
//             "Incredible. I'm looking forward to reading this again"
//         );
//     }

//     #[test]
//     fn test_parse_markdown_modern_post_with_no_review() {
//         let content = r#"\nFulgrim by Graham McNeill\n\n4/5"#;

//         let review = parse_markdown_into_book_review(content).unwrap();

//         assert_eq!(review.title, "Fulgrim");
//         assert_eq!(review.author, "Graham McNeill");
//         assert_eq!(review.score, 4);
//         assert_eq!(review.review, "");
//     }

//     #[test]
//     fn test_parse_microblog_legacy_post() {
//         let content = r#"Finished reading: [The Devastation of Baal](https://oku.club/book/the-devastation-of-baal-by-guy-haley-w22tr) by Guy Haley 📚\n\n4/5 - Excellent book, the descriptions of both the [Blood Angles](https://warhammer40k.fandom.com/wiki/Blood_Angels) and [Tryranids](https://warhammer40k.fandom.com/wiki/Tyranids) are amazing. I would highly recommend it to anyone who's a fan of either faction (or space marines and Warhammer in general).\n"#;

//         let review = parse_markdown_into_book_review(content).unwrap();

//         assert_eq!(review.title, "The Devastation of Baal");
//         assert_eq!(review.author, "Guy Haley");
//         assert_eq!(review.score, 4);
//         assert_eq!(
//             review.review,
//             "Excellent book, the descriptions of both the [Blood Angles](https://warhammer40k.fandom.com/wiki/Blood_Angels) and [Tryranids](https://warhammer40k.fandom.com/wiki/Tyranids) are amazing. I would highly recommend it to anyone who's a fan of either faction (or space marines and Warhammer in general)."
//         );
//     }
// }
