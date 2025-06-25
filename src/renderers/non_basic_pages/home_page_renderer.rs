use askama::Template;

use crate::domain::models::data::Data;
use crate::domain::models::post::Post;
use crate::domain::models::post::PostFilter;
use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::renderers::RendererContext;
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

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
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let updated_at = recent_blog_posts.first().map(|post| post.date);

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
