use askama::Template;
use hypertext::Rendered;

use crate::domain::models::data::Data;
use crate::domain::models::post::Post;
use crate::domain::models::post::PostFilter;
use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderer::partials::page_base::render_page_base;
use crate::renderer::RendererContext;

const RECENT_POSTS_COUNT: usize = 5;

pub async fn render_home_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);

    context
        .renderer
        .render_page(&page.slug, render_page_base(&page, None, None), None)
}
