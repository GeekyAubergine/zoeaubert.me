use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use shared::{cache::CacheDirError, utils::date::DatePaserError};
use tokio::sync::oneshot::error;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::{application::events::Event, infrastructure::bus::job_runner::Job};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("No Authorization header")]
    NoAuthorizationHeader,

    #[error("Invalid header")]
    InvalidHeader,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid token")]
    InvalidToken,
}

impl AuthError {
    pub fn no_authorization_header() -> Error {
        Error::AuthError(AuthError::NoAuthorizationHeader)
    }

    pub fn invalid_header() -> Error {
        Error::AuthError(AuthError::InvalidHeader)
    }

    pub fn unauthorized() -> Error {
        Error::AuthError(AuthError::Unauthorized)
    }

    pub fn invalid_token() -> Error {
        Error::AuthError(AuthError::InvalidToken)
    }

    pub fn into_response(&self) -> ErrorResponse {
        match self {
            AuthError::NoAuthorizationHeader => ErrorResponse {
                status: StatusCode::UNAUTHORIZED,
                message: "No Authorization header",
            },
            AuthError::InvalidHeader => ErrorResponse {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid header",
            },
            AuthError::InvalidToken => ErrorResponse {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid token",
            },
            AuthError::Unauthorized => ErrorResponse {
                status: StatusCode::UNAUTHORIZED,
                message: "Unauthorized",
            },
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        match self {
            AuthError::NoAuthorizationHeader => {
                tonic::Status::unauthenticated("No Authorization header")
            }
            AuthError::InvalidHeader => tonic::Status::unauthenticated("Invalid header"),
            AuthError::InvalidToken => tonic::Status::unauthenticated("Invalid token"),
            AuthError::Unauthorized => tonic::Status::unauthenticated("Unauthorized"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(sqlx::Error),

    #[error("Database query error: {0}")]
    QueryError(sqlx::Error),
}

impl DatabaseError {
    pub fn from_connection_error(err: sqlx::Error) -> Error {
        Error::DatabaseError(DatabaseError::ConnectionError(err))
    }

    pub fn from_query_error(err: sqlx::Error) -> Error {
        Error::DatabaseError(DatabaseError::QueryError(err))
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        tonic::Status::internal("Internal Server Error")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LegoSetsError {
    #[error("Unable to calculate total piece count")]
    UnableToCalculateTotalPieceCount,
    #[error("Unable to calculate total owned count")]
    UnableToCalculateTotalOwnedCount,
}

impl LegoSetsError {
    pub fn unable_to_calculate_total_piece_count() -> Error {
        Error::LegoSetsError(LegoSetsError::UnableToCalculateTotalPieceCount)
    }

    pub fn unable_to_calculate_total_owned_count() -> Error {
        Error::LegoSetsError(LegoSetsError::UnableToCalculateTotalOwnedCount)
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        tonic::Status::internal("Internal Server Error")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LegoMinifiguresError {
    #[error("Unable to calculate total minifigures count")]
    UnableToCalculateTotalMinifiguresCount,
}

impl LegoMinifiguresError {
    pub fn unable_to_calculate_total_minifigures_count() -> Error {
        Error::LegoMinifiguresError(LegoMinifiguresError::UnableToCalculateTotalMinifiguresCount)
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        tonic::Status::internal("Internal Server Error")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Game not found [id: {0}]")]
    GameNotFound(u32),
}

impl GameError {
    pub fn game_not_found(id: u32) -> Error {
        Error::GameError(GameError::GameNotFound(id))
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        tonic::Status::internal("Internal Server Error")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AlbumError {
    #[error("Album not found [id: {0}]")]
    AlbumNotFound(u32),
    #[error("Album photo not found [id: {0}]")]
    AlbumPhotoNotFound(Uuid),
    #[error("Album photo image not found [id: {0}, size: {1}]")]
    AlbumPhotoImageNotFound(Uuid, String),
}

impl AlbumError {
    pub fn album_not_found(id: u32) -> Error {
        Error::AlbumError(AlbumError::AlbumNotFound(id))
    }

    pub fn album_photo_not_found(id: Uuid) -> Error {
        Error::AlbumError(AlbumError::AlbumPhotoNotFound(id))
    }

    pub fn album_photo_image_not_found(id: Uuid, size: String) -> Error {
        Error::AlbumError(AlbumError::AlbumPhotoImageNotFound(id, size))
    }

    pub fn into_response(&self) -> ErrorResponse {
        ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error",
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        tonic::Status::internal("Internal Server Error")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MicroPostError {
    #[error("Post not found {0}")]
    NotFound(String),
}

impl MicroPostError {
    pub fn not_found(slug: String) -> Error {
        Error::MicroPostError(MicroPostError::NotFound(slug))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Couldn't read dir {0}")]
    ReadDir(std::io::Error),
    #[error("Couldn't read config file {0}")]
    ReadConfigFile(std::io::Error),
    #[error("Couldn't parse config file {0}")]
    ParseConfigFile(serde_json::Error),
    #[error("Couldn't make folder {0}")]
    MakeFolder(std::io::Error),
    #[error("HttpReqwest {0}")]
    HttpReqwest(reqwest::Error),
    #[error("CND Upload {0}")]
    CdnUpload(reqwest::Error),
    #[error("CND Download {0}")]
    CdnDownload(reqwest::Error),
    #[error("Unable to parse cdn response {0}")]
    ParseCdnResponse(serde_json::Error),
    #[error("CDN invalid path {0}")]
    CdnInvalidPath(String),
    #[error("CDN file not found {0}")]
    CdnFileNotFound(String),
    #[error("CDN unable to upload file {0} to {1}")]
    CdnUnableToUploadFile(String, String),
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("File system unreadable {0}")]
    FileSystemUnreadable(std::io::Error),
    #[error("File system unwritable {0}")]
    FileSystemUnwritable(std::io::Error),
    #[error("Dispatch job {0}")]
    DispatchJob(tokio::sync::mpsc::error::SendError<Box<dyn Job>>),
    #[error("Dispatch event {0}")]
    DispatchEvent(tokio::sync::mpsc::error::SendError<Event>),
    #[error("Couldn't serialize archive {0}")]
    SerializeArchive(serde_json::Error),
    #[error("Couldn't deserialize archive {0}")]
    DeserializeArchive(serde_json::Error),
    #[error("Couldn't parse blog front matter {0}")]
    ParseBlogFrontMatter(serde_yaml::Error),
    #[error("Couldn't parse micro post front matter {0}")]
    ParseMicroPostFrontMatter(serde_yaml::Error),
    #[error("Couldn't parse blog post {0}")]
    ParseBlogPost(String),
    #[error("Couldn't parse micro post {0}")]
    ParseMicroPost(String),
    #[error("Could not parse date {0}")]
    ParseDate(String),
    #[error("Could not find langauge for code block")]
    CouldNotFindLangaugeForCodeBlock(),
    #[error("Could not find body for code block")]
    CouldNotFindBodyForCodeBlock(),
    #[error("Image size {0}")]
    ImageSize(String),
    #[error("Url download {0}")]
    UrlDownload(reqwest::Error),
    #[error("Parse album {0}")]
    ParseAlbum(serde_yaml::Error),
    #[error("Unable to parse image format {0}")]
    UnableToParseImageFormat(std::io::Error),
    #[error("Unable to decode image {0}")]
    UnableToDecodeImage(image::ImageError),
    #[error("Unable to encode image {0}")]
    UnableToEncodeImage(image::ImageError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("LegoSets error: {0}")]
    LegoSetsError(#[from] LegoSetsError),

    #[error("LegoMinifigures error: {0}")]
    LegoMinifiguresError(#[from] LegoMinifiguresError),

    #[error("Game error: {0}")]
    GameError(#[from] GameError),

    #[error("Album error: {0}")]
    AlbumError(#[from] AlbumError),

    #[error("Auth Error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Micro Post Error: {0}")]
    MicroPostError(#[from] MicroPostError),

    #[error("Date Parser Error: {0}")]
    DateParserError(#[from] DatePaserError),

    #[error("Cache Error: {0}")]
    CacheError(#[from] CacheDirError),

    #[error("Invalid UUID {0}")]
    InvalidUuid(uuid::Error),
}

impl Error {
    pub fn into_response(self) -> ErrorResponse {
        error!("API Error: {:?}", self);

        match self {
            Error::AuthError(e) => e.into_response(),
            Error::AlbumError(e) => e.into_response(),
            Error::DatabaseError(e) => e.into_response(),
            Error::LegoSetsError(e) => e.into_response(),
            Error::LegoMinifiguresError(e) => e.into_response(),
            Error::GameError(e) => e.into_response(),
            _ => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal Server Error",
            },
        }
    }

    pub fn into_tonic_status(self) -> tonic::Status {
        error!("GRPC Error: {:?}", self);

        match self {
            Error::AuthError(e) => e.into_tonic_status(),
            Error::AlbumError(e) => e.into_tonic_status(),
            Error::DatabaseError(e) => e.into_tonic_status(),
            Error::LegoSetsError(e) => e.into_tonic_status(),
            Error::LegoMinifiguresError(e) => e.into_tonic_status(),
            Error::GameError(e) => e.into_tonic_status(),
            _ => tonic::Status::new(tonic::Code::Internal, "Internal Server Error"),
        }
    }
}

pub struct ErrorResponse {
    pub status: StatusCode,
    pub message: &'static str,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        err.into_response()
    }
}

impl From<(reqwest::StatusCode, &'static str)> for ErrorResponse {
    fn from((status, message): (reqwest::StatusCode, &'static str)) -> Self {
        ErrorResponse { status, message }
    }
}
