use std::path::Path;

use crate::prelude::*;

use tracing::debug;

use crate::domain::models::page::Page;
use crate::domain::repositories::Profiler;
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::error::{FileSystemError, TemplateError};

pub mod album_and_photo_pages;
pub mod basic_pages;
pub mod feed_page_renderers;
pub mod formatters;
pub mod home_pages_renderer;
pub mod interest_pages;
pub mod post_pages;
