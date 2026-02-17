use std::sync::Arc;

use askama::Template;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::domain::models::data::Data;
use crate::prelude::*;
use crate::renderer::feeds::render_feeds;
use crate::renderer::pages::albums_pages_renderer::render_albums_pages;
use crate::renderer::pages::blog_pages_renderers::render_blog_pages;
use crate::renderer::pages::book_review_pages_renderers::render_book_review_pages;
use crate::renderer::pages::credits_renderer::render_credits_pages;
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

use crate::error::TemplateError;

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
    render_albums_pages(context)?;
    render_feeds(context)?;
    render_404_page(context)?;
    render_credits_pages(context)?;

    let mut queue = RenderQueue::new();

    render_home_page(&context.data, &mut queue);
    render_blog_pages(&context.data, &mut queue);

    queue
        .tasks
        .into_iter()
        .par_bridge()
        .try_for_each(|task| task.render(&context.renderer))?;

    Ok(())
}

pub type TemplateRenderResult = Result<String>;

pub fn render_template<T: Template>(template: T) -> TemplateRenderResult {
    template.render().map_err(TemplateError::render_error)
}

pub trait RenderTask: Send {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()>;
}

pub struct RenderQueue<'l> {
    tasks: Vec<Box<dyn RenderTask + 'l>>,
}

impl<'l> RenderQueue<'l> {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add(&mut self, task: impl RenderTask + 'l) {
        self.tasks.push(Box::new(task));
    }
}
