use crate::error::{Error, ErrorResponse};
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

pub type ResponseResult<T> = std::result::Result<T, ErrorResponse>;
