use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::MovieError;
use crate::prelude::*;

static LINK_TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[(.*)\]").unwrap());
static MOVIE_YEAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((\d+)(.*\))?").unwrap());
static REVIEW_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"- (.+)$").unwrap());
static SCORE_AND_MAX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\/(\d+)").unwrap());
static NON_LINK_TITLE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*) \((\d+)\)").unwrap());

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MovieReview {
    pub title: String,
    pub year: u16,
    pub score: u8,
    pub review: String,
}

impl MovieReview {
    pub fn from_content(content: &str) -> Result<MovieReview> {
        parse_markdown_into_movie_review(content)
    }
}

// Do not use this directly, use the `movie_service` instead. This is a helper function.
fn parse_markdown_into_movie_review(content: &str) -> Result<MovieReview> {
    let content = content.replace("<p>", "\n").replace("</p>", "\n");

    let lines = content.lines().collect::<Vec<&str>>();

    let lines = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let first_line = lines
        .first()
        .ok_or(MovieError::unable_to_parse_movie(content.to_string()))?;
    let second_line = lines
        .get(1)
        .ok_or(MovieError::unable_to_parse_movie(content.to_string()))?;

    let title = match first_line.starts_with('[') {
        true => LINK_TITLE_REGEX
            .captures(first_line)
            .ok_or(MovieError::unable_to_parse_and_find_movie_title(
                content.to_string(),
            ))?
            .get(1)
            .ok_or(MovieError::unable_to_parse_and_find_movie_title(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
        false => NON_LINK_TITLE_REGEX
            .captures(first_line)
            .ok_or(MovieError::unable_to_parse_and_find_movie_title(
                content.to_string(),
            ))?
            .get(1)
            .ok_or(MovieError::unable_to_parse_and_find_movie_title(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
    };

    let year = MOVIE_YEAR_REGEX
        .captures(first_line)
        .ok_or(MovieError::unable_to_parse_and_find_movie_year(
            content.to_string(),
        ))?
        .get(1)
        .ok_or(MovieError::unable_to_parse_and_find_movie_year(
            content.to_string(),
        ))?
        .as_str()
        .parse::<u16>()
        .map_err(|_| MovieError::unable_to_parse_and_find_movie_year(content.to_string()))?;

    let review = match REVIEW_REGEX.captures(second_line) {
        Some(captures) => captures
            .get(1)
            .ok_or(MovieError::unable_to_parse_and_find_movie_review(
                content.to_string(),
            ))?
            .as_str()
            .to_string(),
        None => "".to_string(),
    };

    let score = SCORE_AND_MAX_REGEX
        .captures(second_line)
        .ok_or(MovieError::unable_to_parse_and_find_movie_score(
            content.to_string(),
        ))?
        .get(1)
        .ok_or(MovieError::unable_to_parse_and_find_movie_score(
            content.to_string(),
        ))?
        .as_str()
        .parse::<u8>()
        .map_err(|_| MovieError::unable_to_parse_and_find_movie_score(content.to_string()))?;

    Ok(MovieReview {
        title,
        year,
        score,
        review,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_legacy_micro_blog_format() {
        let post = "[Chicken Little](https://www.imdb.com/title/tt0371606/) (2005) 🍿\n\n3/5 - Nice easy watch, some good moments and laughs\n";

        let expected = MovieReview {
            title: "Chicken Little".to_string(),
            year: 2005,
            score: 3,
            review: "Nice easy watch, some good moments and laughs".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        dbg!(&review);

        assert_eq!(review, expected);
    }

    #[test]
    fn it_should_parse_legacy_micro_blog_format_with_no_review() {
        let post = "[Desert Hearts](https://www.imdb.com/title/tt0089015/) (1985) 🍿\n\n3/5\n";

        let expected = MovieReview {
            title: "Desert Hearts".to_string(),
            year: 1985,
            score: 3,
            review: "".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        assert_eq!(review, expected)
    }

    #[test]
    fn it_should_parse_legacy_micro_blog_format_with_custom_metadata_in_year() {
        let post = "[The Blues Brothers](https://www.imdb.com/title/tt0080455/) (1980 - Extended Version) 🍿\n\n5/5 - This film gets better every time I watch it.\n";

        let expected = MovieReview {
            title: "The Blues Brothers".to_string(),
            year: 1980,
            score: 5,
            review: "This film gets better every time I watch it.".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        assert_eq!(review.title, expected.title);
    }

    #[test]
    fn it_should_parse_micro_post_format() {
        let post = "[All Quiet on the Western Front](https://www.imdb.com/title/tt1016150/) (2022)\n\n3/5 - I see why others enjoyed it, but a lot of it felt like gore for the sake of gore. The performances are great.";

        let expected = MovieReview {
            title: "All Quiet on the Western Front".to_string(),
            year: 2022,
            score: 3,
            review: "I see why others enjoyed it, but a lot of it felt like gore for the sake of gore. The performances are great.".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        assert_eq!(review.title, expected.title);
    }

    #[test]
    fn it_should_parse_mastodon_post_format() {
        let post = "<p>The Menu (2022)</p><p>2/5 - Interesting, but not for me</p>";

        let expected = MovieReview {
            title: "The Menu".to_string(),
            year: 2022,
            score: 2,
            review: "Interesting, but not for me".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        assert_eq!(review.title, expected.title);
    }

    #[test]
    fn it_should_parse_mastodon_post_format_with_no_review() {
        let post = "<p>Yentl (1983)</p><p>3/5</p>";

        let expected = MovieReview {
            title: "Yentl".to_string(),
            year: 1983,
            score: 3,
            review: "".to_string(),
        };

        let review = parse_markdown_into_movie_review(&post).unwrap();

        assert_eq!(review.title, expected.title);
    }
}
