use askama::Template;

use crate::{
    domain::models::{
        page::{Page, PagePagination},
        post::Post,
        post::PostFilter,
        slug::Slug,
    },
    renderers::RendererContext,
    utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "posts/post_page/post_page.html")]
pub struct PostTemplate {
    page: Page,
    post: Post,
}

pub async fn render_post_pages(context: &RendererContext) -> Result<()> {
    let omni_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_all());

    for omni_post in omni_posts {
        render_post_page(context, &omni_post).await?;
    }

    Ok(())
}

async fn render_post_page(context: &RendererContext, post: &Post) -> Result<()> {
    match post.page() {
        Some(page) => {
            let template = PostTemplate {
                page: page.clone(),
                post: post.clone(),
            };

            context
                .renderer
                .render_page(&template.page.slug, &template, Some(post.date().clone()))
                .await
        }
        None => Ok(()),
    }
}
