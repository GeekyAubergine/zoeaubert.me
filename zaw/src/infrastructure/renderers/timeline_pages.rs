use askama::Template;

use crate::{
    domain::{
        models::{
            omni_post::OmniPost,
            page::{Page, PagePagination},
            slug::Slug,
        },
        state::State,
    },
    infrastructure::utils::paginator::PaginatorPage,
};

use crate::prelude::*;

use super::render_page_with_template;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "timeline/index.html")]
pub struct TimelineTemplate<'t> {
    page: &'t Page<'t>,
    posts: &'t [OmniPost],
}

pub async fn render_timeline_page<'d>(
    state: &impl State,
    paginator_page: &PaginatorPage<'d, OmniPost>,
) -> Result<()> {
    let page = Page::new(Slug::new("timeline"), Some("Timeline"), Some("My timeline"))
        .with_pagination_from_paginator(paginator_page, "Posts");

    let template = TimelineTemplate {
        page: &page,
        posts: paginator_page.data,
    };

    render_page_with_template(state, &page, template).await
}
