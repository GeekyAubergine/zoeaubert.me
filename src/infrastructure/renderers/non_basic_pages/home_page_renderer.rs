use askama::Template;

use crate::domain::models::data::{Data, PostFilter};
use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::domain::queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};
use crate::domain::repositories::{AboutTextRepo, BlogPostsRepo, SillyNamesRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::state::State;
use crate::infrastructure::renderers::RendererContext;
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
    recent_blog_posts: Vec<&'t BlogPost>,
}

pub async fn render_home_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);

    let recent_blog_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::BLOG_POST)
        .iter()
        .take(RECENT_POSTS_COUNT)
        .filter_map(|post| match post {
            OmniPost::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let updated_at = recent_blog_posts.first().map(|post| post.updated_at);

    let template = IndexTemplate {
        page,
        data: &context.data,
        recent_blog_posts,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, updated_at)
        .await
}
