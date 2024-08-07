use reqwest::StatusCode;

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
pub enum SillyNamesError {
    #[error("Unable to read csv {0}")]
    UnableToReadCsv(csv::Error),

    #[error("Unable to parse csv {0}")]
    UnableToParseCsv(csv::Error),
}

impl SillyNamesError {
    pub fn unable_to_read_csv(error: csv::Error) -> Error {
        Error::SillyNames(Self::UnableToReadCsv(error))
    }

    pub fn unable_to_parse_csv(error: csv::Error) -> Error {
        Error::SillyNames(Self::UnableToParseCsv(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ApiError error: {0}")]
    ApiError(ApiError),
    #[error("Silly names error: {0}")]
    SillyNames(SillyNamesError),
    #[error("Tonic error: {0}")]
    Tonic(TonicError),

    #[error("HttpReqwest {0}")]
    HttpReqwest(reqwest::Error),
}
