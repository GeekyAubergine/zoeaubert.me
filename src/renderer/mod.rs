use std::path::Path;
use std::sync::Arc;

use askama::Template;

use crate::domain::models::data::Data;
use crate::prelude::*;
use crate::renderer::feeds::{render_feeds};
use crate::renderer::pages::albums_pages_renderer::render_alubms_pages;
use crate::renderer::pages::blog_pages_renderers::render_blog_pages;
use crate::renderer::pages::book_review_pages_renderers::render_book_review_pages;
use crate::renderer::pages::faq_page_renderer::render_faq_page;
use crate::renderer::pages::feeds_page_renderer::render_feeds_page;
use crate::renderer::pages::firehose_pages_renderers::render_firehose_pages;
use crate::renderer::pages::four_o_four_renderer::render_404_page;
use crate::renderer::pages::games_pages_renderers::render_games_pages;
use crate::renderer::pages::home_page_renderer::render_home_page;
use crate::renderer::pages::interests_page_renderer::render_interests_page;
use crate::renderer::pages::lego_pages_renderers::render_lego_pages;
use crate::renderer::pages::mastodon_post_pages_renderers::render_mastodon_pages;
use crate::renderer::pages::micro_post_pages_renderers::render_micro_post_pages;
use crate::renderer::pages::movie_review_pages_renderers::render_move_review_pages;
use crate::renderer::pages::now_page_renderer::render_now_page;
use crate::renderer::pages::photo_pages_renderer::render_photo_pages;
use crate::renderer::pages::project_pages_renderers::render_project_pages;
use crate::renderer::pages::referrals_page_renderer::render_referrals_page;
use crate::renderer::pages::support_page_renderer::render_support_page;
use crate::renderer::pages::tag_pages_renderers::render_tags_pages;
use crate::renderer::pages::timeline_pages_renderers::render_timeline_pages;
use crate::renderer::pages::tv_review_pages_renderers::render_tv_review_pages;
use crate::services::page_renderer::PageRenderer;
use tracing::debug;

use crate::domain::models::page::Page;
use crate::error::{FileSystemError, TemplateError};

pub mod formatters;

pub mod feeds;
pub mod pages;
pub mod partials;

pub struct RendererContext {
    pub data: Data,
    pub renderer: PageRenderer,
}

pub fn new_rendering_context_from_data(data: Data) -> Result<RendererContext> {
    Ok(RendererContext {
        data,
        renderer: PageRenderer::new(),
    })
}

pub fn render_pages(context: &RendererContext) -> Result<()> {
    render_home_page(context)?;
    render_blog_pages(context)?;
    render_micro_post_pages(context)?;
    render_mastodon_pages(context)?;
    render_photo_pages(context)?;
    render_timeline_pages(context)?;
    render_tags_pages(context)?;
    render_firehose_pages(context)?;
    render_project_pages(context)?;
    render_interests_page(context)?;
    render_book_review_pages(context)?;
    render_move_review_pages(context)?;
    render_tv_review_pages(context)?;
    render_games_pages(context)?;
    render_lego_pages(context)?;
    render_now_page(context)?;
    render_faq_page(context)?;
    render_support_page(context)?;
    render_referrals_page(context)?;
    render_feeds_page(context)?;
    render_alubms_pages(context)?;
    render_feeds(context)?;
    render_404_page(context)?;

    Ok(())
}

pub type TemplateRenderResult = Result<String>;

pub fn render_template<T: Template>(template: T) -> TemplateRenderResult {
    template.render().map_err(TemplateError::render_error)
}
