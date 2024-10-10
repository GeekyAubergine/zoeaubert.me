use askama::Template;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::{render_page_with_template};

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct BlogsListTemplate<'t, 'p> {
    page: &'t Page<'p>,
    blog_posts: Vec<BlogPost>,
}

pub async fn render_blogs_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/blog"),
        Some("Blog Posts"),
        Some("My blog posts"),
    );

    let blog_posts = find_all_blog_posts(state).await?;

    let template = BlogsListTemplate {
        page: &page,
        blog_posts,
    };

    render_page_with_template(&page, template).await
}
