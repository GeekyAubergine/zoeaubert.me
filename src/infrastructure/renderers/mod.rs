use std::path::Path;

use crate::prelude::*;

use tracing::debug;

use crate::domain::models::page::Page;
use crate::domain::repositories::Profiler;
use crate::domain::services::FileService;
use crate::domain::state::State;
use crate::error::{FileSystemError, TemplateError};

pub mod albums_and_photos_renderer;
pub mod blog_pages_renderer;
pub mod faq_page_renderer;
pub mod feed_renderers;
pub mod formatters_renderer;
pub mod games_pages_renderer;
pub mod home_page_renderer;
pub mod interests_page_renderer;
pub mod lego_pages_renderer;
pub mod mastodon_post_pages_renderers;
pub mod micro_post_pages_renderers;
pub mod movie_pages_renderer;
pub mod now_page_renderer;
pub mod photo_pages_renderer;
pub mod save_pages_renderer;
pub mod tags_pages_renderer;
pub mod timeline_renderer;
pub mod tv_show_renderer;
pub mod years_renderer;
