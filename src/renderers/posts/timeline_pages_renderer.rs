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

use tokio::try_join;

const DEFAULT_PAGINATION_SIZE: usize = 25;

#[derive(Template)]
#[template(path = "posts/post_list/post_list_page.html")]
pub struct TimelineTemplate {
    page: Page,
    posts: Vec<Post>,
}

pub async fn render_timeline_pages(context: &RendererContext) -> Result<()> {
    try_join!(render_timeline_page(context), render_firehost_page(context))?;

    Ok(())
}

async fn render_timeline_page(context: &RendererContext) -> Result<()> {
    let omni_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_main_timeline());

    let paginated = paginate(&omni_posts, DEFAULT_PAGINATION_SIZE);

    let page = Page::new(
        Slug::new("timeline"),
        Some("Timeline"),
        Some("My timeline".to_string()),
    );

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = TimelineTemplate {
            page,
            posts: paginator_page
                .data
                .iter()
                .cloned()
                .cloned()
                .collect::<Vec<Post>>(),
        };

        let updated_at = paginator_page.data.first().map(|p| p.date().clone());

        context
            .renderer
            .render_page(&template.page.slug, &template, updated_at)
            .await?;
    }

    Ok(())
}

async fn render_firehost_page(context: &RendererContext) -> Result<()> {
    let omni_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_firehose());

    let paginated = paginate(&omni_posts, 100);

    let page = Page::new(
        Slug::new("firehose"),
        Some("Firehose"),
        Some("All the things, all the time".to_string()),
    );

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = TimelineTemplate {
            page,
            posts: paginator_page
                .data
                .iter()
                .cloned()
                .cloned()
                .collect::<Vec<Post>>(),
        };

        let updated_at = paginator_page.data.first().map(|p| p.date().clone());

        context
            .renderer
            .render_page(&template.page.slug, &template, updated_at)
            .await?;
    }

    Ok(())
}
