use askama::Template;

use crate::{
    domain::{
        models::{
            omni_post::OmniPost,
            page::{Page, PagePagination},
            slug::Slug,
        }, queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags}, state::State
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

#[derive(Template)]
#[template(path = "timeline/index.html")]
pub struct TimelineTemplate<'t> {
    page: &'t Page<'t>,
    posts: &'t [OmniPost],
}

pub async fn render_timeline_page<'d>(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_main_timeline()).await?;

    let paginated = paginate(&omni_posts, DEFAULT_PAGINATION_SIZE);

    for paginator_page in paginated {
        let page = Page::new(Slug::new("timeline"), Some("Timeline"), Some("My timeline"))
            .with_pagination_from_paginator(&paginator_page, "Posts");

        let template = TimelineTemplate {
            page: &page,
            posts: paginator_page.data,
        };

        render_page_with_template(state, &page, template).await?;
    }

    Ok(())
}
