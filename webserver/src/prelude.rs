use crate::error::Error;
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

pub type TemplateErrorResponse = (StatusCode, &'static str);

pub type TemplateResult<T> = std::result::Result<T, TemplateErrorResponse>;
