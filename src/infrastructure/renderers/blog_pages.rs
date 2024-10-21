use askama::Template;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::render_page_with_template;

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct BlogsListTemplate<'t> {
    page: &'t Page<'t>,
    blog_posts: &'t[BlogPost],
}

pub async fn render_blogs_list_page(state: &impl State, blog_posts: &[BlogPost]) -> Result<()> {
    let page = Page::new(
        Slug::new("/blog"),
        Some("Blog Posts"),
        Some("My blog posts"),
    );

    let template = BlogsListTemplate {
        page: &page,
        blog_posts,
    };

    render_page_with_template(state, &page, template).await
}

#[derive(Template)]
#[template(path = "blog/post.html")]
pub struct BlogPostTemplate<'t> {
    page: &'t Page<'t>,
    post: &'t BlogPost,
}

pub async fn render_blog_post_page(state: &impl State, blog_post: &BlogPost) -> Result<()> {
    let page = Page::new(
        blog_post.slug.clone(),
        Some(&blog_post.title),
        Some(&blog_post.description),
    )
    .with_date(blog_post.date)
    .with_tags(blog_post.tags.clone());

    let template = BlogPostTemplate {
        page: &page,
        post: blog_post,
    };

    render_page_with_template(state, &page, template).await
}
