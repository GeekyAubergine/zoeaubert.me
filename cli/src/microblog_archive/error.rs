use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum MicroBlogArchiveError {
    #[error("Unable to read file {0}")]
    UnableToParseFile(serde_json::Error),
}

impl MicroBlogArchiveError {
    pub fn unable_to_parse_file(error: serde_json::Error) -> Error {
        Error::MicroBlogArchive(Self::UnableToParseFile(error))
    }
}
