use askama::Template;

use crate::domain::models::data::Data;
use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::domain::queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};
use crate::domain::repositories::{AboutTextRepo, BlogPostsRepo, SillyNamesRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::state::State;
use crate::infrastructure::renderers::RendererContext;
use crate::infrastructure::services::page_renderer::PageRenderer;
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const RECENT_POSTS_COUNT: usize = 5;

#[derive(Template)]
#[template(path = "home_page.html")]
pub struct IndexTemplate<'t> {
    page: Page,
    data: &'t Data,
    recent_blog_posts: Vec<BlogPost>,
    recent_omni_posts: Vec<OmniPost>,
}

pub async fn render_home_page(state: &impl State, context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);

    let recent_blog_posts = state
        .blog_posts_repo()
        .find_all_by_date()
        .await?
        .iter()
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    let most_recent_post = recent_blog_posts.first().cloned();

    let recent_omni_posts = find_all_omni_posts(state, OmniPostFilterFlags::filter_home_page())
        .await?
        .iter()
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<OmniPost>>();

    let template = IndexTemplate {
        page,
        data: context.data(),
        recent_blog_posts,
        recent_omni_posts,
    };

    let updated_at = most_recent_post.map(|p| p.updated_at);

    context
        .renderer
        .render_page(state, &template.page.slug, &template, updated_at.clone())
        .await
}
