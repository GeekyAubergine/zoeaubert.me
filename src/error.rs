#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unable to find file {0}")]
    UnableToFindFile(String),
    #[error("Unable to read file {0}")]
    UnableToReadFile(String),
    #[error("Unable to write file {0}")]
    UnableToWriteFile(String),
    #[error("Unable to parse config file {0}")]
    UnableToParseConfigFile(String),
    #[error("Unable to parse mastodon posts cache {0}")]
    UnableToParseMastodonPostsCache(String),
}