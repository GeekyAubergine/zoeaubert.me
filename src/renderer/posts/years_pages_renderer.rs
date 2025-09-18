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

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_years_pages<'d>(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_year_and_grouped_by_year(PostFilter::filter_main_timeline());

    for (year, posts) in posts {
        render_year_pages(context, year, &posts).await?;
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "posts/post_list/post_list_page.html")]
pub struct YearTemplate {
    page: Page,
    year: u16,
    posts: Vec<Post>,
}

async fn render_year_pages<'d>(
    context: &RendererContext,
    year: u16,
    posts: &[&Post],
) -> Result<()> {
    let page = Page::new(
        Slug::new(&format!("/years/{}", year)),
        Some(&format!("{} posts", year)),
        Some(format!("My {} posts", year)),
    );

    for paginator_page in paginate(posts, DEFAULT_PAGINATION_SIZE) {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = YearTemplate {
            page,
            year,
            posts: paginator_page.data.iter().cloned().cloned().collect(),
        };

        context
            .renderer
            .render_page(&template.page.slug, &template, None)
            .await?;
    }

    Ok(())
}
