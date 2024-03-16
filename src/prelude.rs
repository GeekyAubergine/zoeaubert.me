use crate::error::Error;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;
