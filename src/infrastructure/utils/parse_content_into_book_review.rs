use std::ptr::replace;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    domain::models::{content::Content, omni_post::OmniPost},
    error::{BookError, TvShowsError},
    prelude::*,
};

const REGEX_MICRO: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(.+) by (.+)+\s*(\d)/\d+ - (.+)$").unwrap());

const REGEX_LEGACY_STYLE_AUTHOR_TITLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[(.*)\].*by (.*) 📚").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub title: String,
    pub author: String,
    pub score: u8,
    pub review: String,
}

pub fn parse_content_into_book_review(content: &Content) -> Result<Review> {
    match content {
        Content::MicroPost(post) => parse_markdown_into_book_review(&post.content),
        Content::MastodonPost(post) => parse_markdown_into_book_review(&post.content()),
        _ => Err(TvShowsError::unsupported_content_type(content)),
    }
}

fn parse_new_style(content: &str) -> Result<Review> {
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
            return Ok(Review {
                title: title.trim().to_string(),
                author: author.trim().to_string(),
                score,
                review: review.trim().to_string(),
            });
        }
    }

    Err(BookError::unable_to_parse_book(content.to_string()))
}

fn parse_mastodon_modern_post(content: &str) -> Result<Review> {
    let content = content
        .replace("</p><p>", "\n")
        .replace("<p>", "")
        .replace("</p>", "");

    parse_new_style(&content)
}

fn parse_legacy_post(content: &str) -> Result<Review> {
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
            return Ok(Review {
                title: title.as_str().to_string(),
                author: author.as_str().to_string(),
                score,
                review: review.trim().to_string(),
            });
        }
    }

    Err(BookError::unable_to_parse_book(content.to_string()))
}

// Do not use this directly, use the `books service` instead. This is a helper function.
fn parse_markdown_into_book_review(content: &str) -> Result<Review> {
    println!("");

    println!("Parsing book review: [{:?}]", content);

    println!("");

    if (content.starts_with("Finished") || content.starts_with("I finished")) {
        return parse_legacy_post(content);
    }

    if (content.starts_with("<p>")) {
        return parse_mastodon_modern_post(content);
    }

    parse_new_style(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mastodon_modern_post() {
        let content = r#"<p>Legion by Dan Abnett</p><p>2/5 - The story is good, but I found it hard to follow in places.</p>"#;

        let review = parse_markdown_into_book_review(content).unwrap();

        assert_eq!(review.title, "Legion");
        assert_eq!(review.author, "Dan Abnett");
        assert_eq!(review.score, 2);
        assert_eq!(
            review.review,
            "The story is good, but I found it hard to follow in places."
        );
    }

    #[test]
    fn test_parse_markdown_modern_post() {
        let content = r#"The First Heretic by Aaron Dembski-Bowden

        4/5 - Incredible. I'm looking forward to reading this again"#;

        let review = parse_markdown_into_book_review(content).unwrap();

        assert_eq!(review.title, "The First Heretic");
        assert_eq!(review.author, "Aaron Dembski-Bowden");
        assert_eq!(review.score, 4);
        assert_eq!(
            review.review,
            "Incredible. I'm looking forward to reading this again"
        );
    }

    #[test]
    fn test_parse_markdown_modern_post_with_no_review() {
        let content = r#"\nFulgrim by Graham McNeill\n\n4/5"#;

        let review = parse_markdown_into_book_review(content).unwrap();

        assert_eq!(review.title, "Fulgrim");
        assert_eq!(review.author, "Graham McNeill");
        assert_eq!(review.score, 4);
        assert_eq!(review.review, "");
    }

    #[test]
    fn test_parse_microblog_legacy_post() {
        let content = r#"Finished reading: [The Devastation of Baal](https://oku.club/book/the-devastation-of-baal-by-guy-haley-w22tr) by Guy Haley 📚\n\n4/5 - Excellent book, the descriptions of both the [Blood Angles](https://warhammer40k.fandom.com/wiki/Blood_Angels) and [Tryranids](https://warhammer40k.fandom.com/wiki/Tyranids) are amazing. I would highly recommend it to anyone who's a fan of either faction (or space marines and Warhammer in general).\n"#;

        let review = parse_markdown_into_book_review(content).unwrap();

        assert_eq!(review.title, "The Devastation of Baal");
        assert_eq!(review.author, "Guy Haley");
        assert_eq!(review.score, 4);
        assert_eq!(
            review.review,
            "Excellent book, the descriptions of both the [Blood Angles](https://warhammer40k.fandom.com/wiki/Blood_Angels) and [Tryranids](https://warhammer40k.fandom.com/wiki/Tyranids) are amazing. I would highly recommend it to anyone who's a fan of either faction (or space marines and Warhammer in general)."
        );
    }
}
