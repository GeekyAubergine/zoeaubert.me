use crate::error::Error;

pub mod book_review;
pub mod movie_review;
pub mod review_source;
pub mod tv_show_review;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReviewScore {
    Positive,
    Neutral,
    Negative,
}

impl ReviewScore {
    pub fn as_emoji(&self) -> &str {
        match self {
            ReviewScore::Negative => "👎",
            ReviewScore::Neutral => "🤷‍♀️",
            ReviewScore::Positive => "👍",
        }
    }
}

impl TryFrom<u8> for ReviewScore {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 | 1 => Ok(ReviewScore::Negative),
            2 | 3 => Ok(ReviewScore::Neutral),
            4 | 5 => Ok(ReviewScore::Positive),
            _ => Err(Error::InvalidReviewScore()),
        }
    }
}

impl TryFrom<&str> for ReviewScore {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "👎" => Ok(ReviewScore::Negative),
            "🤷‍♀️" => Ok(ReviewScore::Neutral),
            "👍" => Ok(ReviewScore::Positive),
            _ => Err(Error::InvalidReviewScore()),
        }
    }
}
