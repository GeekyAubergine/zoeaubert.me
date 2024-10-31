use askama::Template;

use crate::{
    domain::{
        models::{
            omni_post::OmniPost,
            page::{Page, PagePagination},
            slug::Slug,
        },
        queries::omni_post_queries::{
            find_all_omni_posts, find_omni_posts_grouped_by_year, OmniPostFilterFlags,
        },
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use super::render_page_with_template;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_years_pages<'d>(state: &impl State) -> Result<()> {
    let posts =
        find_omni_posts_grouped_by_year(state, OmniPostFilterFlags::filter_main_timeline()).await?;

    for (year, posts) in posts {
        render_year_pages(state, year, &posts).await?;
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "years/year.html")]
pub struct YearTemplate<'t> {
    page: &'t Page<'t>,
    year: u16,
    posts: &'t [OmniPost],
}

async fn render_year_pages<'d>(state: &impl State, year: u16, posts: &[OmniPost]) -> Result<()> {
    for paginator_page in paginate(posts, DEFAULT_PAGINATION_SIZE) {
        let page = Page::new(
            Slug::new(&format!("/years/{}", year)),
            Some(&format!("{} posts", year)),
            Some(&format!("My {} posts", year)),
        )
        .with_pagination_from_paginator(&paginator_page, "Posts");

        let template = YearTemplate {
            page: &page,
            year,
            posts: paginator_page.data,
        };

        render_page_with_template(state, &page, template).await?;
    }

    Ok(())
}
