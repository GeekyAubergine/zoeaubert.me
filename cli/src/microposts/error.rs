use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum MicroPostError {
    #[error("Unable to parse front matter {0}")]
    UnableToParseFrontMatter(serde_yaml::Error),

    #[error("No front matter found in file {0}")]
    NoFrontMatter(String),

    #[error("Invalid file path {0}")]
    InvalidFilePath(String),

    #[error("Invalid file name {0}")]
    InvalidFileName(String),

    #[error("No content found in file {0}")]
    NoContent(String),
}

impl MicroPostError {
    pub fn unable_to_parse_front_matter(error: serde_yaml::Error) -> Error {
        Error::MicroPost(Self::UnableToParseFrontMatter(error))
    }

    pub fn no_front_matter(path: String) -> Error {
        Error::MicroPost(Self::NoFrontMatter(path))
    }

    pub fn invalid_file_path(path: String) -> Error {
        Error::MicroPost(Self::InvalidFilePath(path))
    }

    pub fn invalid_file_name(name: String) -> Error {
        Error::MicroPost(Self::InvalidFileName(name))
    }

    pub fn no_content(name: String) -> Error {
        Error::MicroPost(Self::NoContent(name))
    }
}
