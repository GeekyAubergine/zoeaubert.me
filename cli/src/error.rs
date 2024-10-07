use reqwest::StatusCode;
use shared::utils::date::DatePaserError;

use crate::{
    microblog_archive::error::MicroBlogArchiveError, microposts::error::MicroPostError,
    silly_names::error::SillyNamesError,
};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid header config: {0}")]
    InvalidHeaderConfig(String),

    #[error("Bad response {0} {1}")]
    BadResponse(StatusCode, String),
}

impl ApiError {
    pub fn invalid_header_config(header: String) -> Error {
        Error::ApiError(Self::InvalidHeaderConfig(header))
    }

    pub fn bad_response(status: StatusCode, url: String) -> Error {
        Error::ApiError(Self::BadResponse(status, url))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TonicError {
    #[error("Unable to connect {0}")]
    UnableToConnect(tonic::transport::Error),

    #[error("Server returned status {0}")]
    ServerReturnedStatus(tonic::Status),

    #[error("Invalid URI {0}")]
    InvalidUri(http::uri::InvalidUri),
}

impl TonicError {
    pub fn unable_to_connect(error: tonic::transport::Error) -> Error {
        Error::Tonic(Self::UnableToConnect(error))
    }

    pub fn server_returned_status(status: tonic::Status) -> Error {
        Error::Tonic(Self::ServerReturnedStatus(status))
    }

    pub fn invalid_uri(error: http::uri::InvalidUri) -> Error {
        Error::Tonic(Self::InvalidUri(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("Unable to read file {0}")]
    UnableToReadFile(std::io::Error),

    #[error("Unable to write file {0}")]
    UnableToWriteFile(std::io::Error),

    #[error("Unable to read dir {0}")]
    UnableToReadDir(std::io::Error),
}

impl FileSystemError {
    pub fn unable_to_read_file(error: std::io::Error) -> Error {
        Error::FileSystem(Self::UnableToReadFile(error))
    }

    pub fn unable_to_write_file(error: std::io::Error) -> Error {
        Error::FileSystem(Self::UnableToWriteFile(error))
    }

    pub fn unable_to_read_dir(error: std::io::Error) -> Error {
        Error::FileSystem(Self::UnableToReadDir(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ApiError error: {0}")]
    ApiError(ApiError),
    #[error("Tonic error: {0}")]
    Tonic(TonicError),
    #[error("FileSystem error: {0}")]
    FileSystem(FileSystemError),

    #[error("MicroBlogArchive error: {0}")]
    MicroBlogArchive(MicroBlogArchiveError),
    #[error("Silly names error: {0}")]
    SillyNames(SillyNamesError),
    #[error("MicroPost error: {0}")]
    MicroPost(MicroPostError),

    #[error("HttpReqwest {0}")]
    HttpReqwest(reqwest::Error),
    #[error("Date parse error {0}")]
    DateParse(DatePaserError),
}
