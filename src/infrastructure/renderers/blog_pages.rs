use askama::Template;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::BlogPostsRepo;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use super::render_page_with_template;

pub async fn render_blog_pages(state: &impl State) -> Result<()> {
    let blog_posts = state.blog_posts_repo().find_all_by_date().await?;

    render_blogs_list_page(state, &blog_posts).await?;

    for blog_post in blog_posts {
        render_blog_post_page(state, &blog_post).await?;
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "blog/index.html")]
struct BlogsListTemplate<'t> {
    page: &'t Page<'t>,
    blog_posts: &'t[BlogPost],
}

async fn render_blogs_list_page(state: &impl State, blog_posts: &[BlogPost]) -> Result<()> {
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
struct BlogPostTemplate<'t> {
    page: &'t Page<'t>,
    post: &'t BlogPost,
}

async fn render_blog_post_page(state: &impl State, blog_post: &BlogPost) -> Result<()> {
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
