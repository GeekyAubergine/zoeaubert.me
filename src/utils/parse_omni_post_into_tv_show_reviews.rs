use once_cell::sync::Lazy;
use regex::Regex;

use crate::{domain::models::{raw_content::SourcePost, post::Post}, error::TvShowsError, prelude::*};

const LINK_TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[(.*)\]").unwrap());
const SEASON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((S.*)\)").unwrap());
const REVIEW_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"- (.+)$").unwrap());
const SCORE_AND_MAX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\/(\d+)").unwrap());
const NON_LINK_TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) \((.+)\)").unwrap());

const SIMPLE_SEASON_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(S(\d+)\)").unwrap());
const NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub title: String,
    pub seasons: Vec<u8>,
    pub scores: Vec<u8>,
    pub review: String,
}

pub fn parse_content_into_tv_show_review(content: &SourcePost) -> Result<Review> {
    match content {
        SourcePost::MicroPost(post) => parse_markdown_into_tv_show_review(&post.content),
        SourcePost::MastodonPost(post) => parse_markdown_into_tv_show_review(&post.content()),
        _ => Err(TvShowsError::unsupported_content_type(content)),
    }
}

// Do not use this directly, use the `tv_service` instead. This is a helper function.
fn parse_markdown_into_tv_show_review(content: &str) -> Result<Review> {
    let content = content.replace("<p>", "\n").replace("</p>", "\n");

    let lines = content.lines().collect::<Vec<&str>>();

    let lines = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let first_line = lines
        .first()
        .ok_or(TvShowsError::unable_to_parse_tv_show(content.to_string()))?;
    let second_line = lines
        .get(1)
        .ok_or(TvShowsError::unable_to_parse_tv_show(content.to_string()))?;

    let title = match first_line.starts_with('[') {
        true => LINK_TITLE_REGEX
            .captures(first_line)
            .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_title(
                content.to_string(),
            ))?
            .get(1)
            .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_title(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
        false => NON_LINK_TITLE_REGEX
            .captures(first_line)
            .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_title(
                content.to_string(),
            ))?
            .get(1)
            .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_title(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
    };

    let seasons = SEASON_REGEX
        .captures(first_line)
        .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_season(
            content.to_string(),
        ))?
        .get(1)
        .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_season(
            content.to_string(),
        ))?
        .as_str();

    let seasons = NUMBERS_REGEX
        .find_iter(seasons)
        .map(|m| m.as_str().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let review = match REVIEW_REGEX.captures(second_line) {
        Some(captures) => captures
            .get(1)
            .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_review(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
        None => "".to_string(),
    };

    let score = SCORE_AND_MAX_REGEX
        .captures(second_line)
        .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_score(
            content.to_string(),
        ))?
        .get(1)
        .ok_or(TvShowsError::unable_to_parse_and_find_tv_show_score(
            content.to_string(),
        ))?
        .as_str()
        .parse::<u8>()
        .map_err(|_| TvShowsError::unable_to_parse_and_find_tv_show_score(content.to_string()))?;

    Ok(Review {
        title: title.clone(),
        seasons: seasons.clone(),
        scores: seasons.iter().map(|_| score).collect::<Vec<u8>>(),
        review: review.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_should_parse_legacy_micro_blog_format_with_multi_season() {
        let content= "[F Is for Family](https://www.imdb.com/title/tt4326894/) (Seasons 3, 4 & 5) 📺\n\n4/5 - The show continues to improve. The last two seasons touch on much more serious subjects and the show really shines for it.\n";

        let expected = Review {
            title: "F Is for Family".to_string(),
            scores: vec![4, 4, 4],
            seasons: vec![3, 4, 5],
            review: "The show continues to improve. The last two seasons touch on much more serious subjects and the show really shines for it.".to_string(),
        };

        let result = parse_markdown_into_tv_show_review(content).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_parse_mastodon_format() {
        let content = "<p>Game of Thrones (S7)</p><p>2/5 - The worst so far. Let&#39;s see how bad S8 is.</p>";

        let expected = Review {
            title: "Game of Thrones".to_string(),
            scores: vec![2],
            seasons: vec![7],
            review: "The worst so far. Let&#39;s see how bad S8 is.".to_string(),
        };

        let result = parse_markdown_into_tv_show_review(content).unwrap();

        assert_eq!(result, expected);
    }
}
