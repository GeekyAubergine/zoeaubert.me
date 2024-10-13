use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, slug::Slug},
        state::State,
    },
    infrastructure::utils::paginator::PaginatorPage,
    prelude::*,
};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

use super::render_page_with_template;

pub struct TagCount {
    pub tag: Tag,
    pub count: usize,
}

#[derive(Template)]
#[template(path = "tags/index.html")]
pub struct TagsListTemplate<'t> {
    page: &'t Page<'t>,
    tags: Vec<TagCount>,
}

pub async fn render_tags_list_page(
    state: &impl State,
    tag_counts: &HashMap<Tag, usize>,
) -> Result<()> {
    let mut tags = tag_counts
        .into_iter()
        .map(|(tag, count)| TagCount {
            tag: tag.clone(),
            count: *count,
        })
        .collect::<Vec<TagCount>>();
    tags.sort_by(|a, b| a.tag.cmp(&b.tag));

    let page = Page::new(Slug::new("tags"), Some("Tags"), Some("All Tags"));

    let template = TagsListTemplate { page: &page, tags };

    render_page_with_template(state, &page, template).await
}

#[derive(Template)]
#[template(path = "tags/tag.html")]
pub struct TagPostsTemplate<'t> {
    page: &'t Page<'t>,
    posts: &'t [OmniPost],
}

pub async fn render_tag_page<'d>(
    state: &impl State,
    tag: &Tag,
    paginator_page: &PaginatorPage<'d, OmniPost>,
) -> Result<()> {
    let page = Page::new(
        Slug::new(&format!("tags/{}", tag.slug())),
        Some(&format!("{} Posts", tag.title())),
        Some(&format!("#{} posts", tag.title())),
    )
    .with_pagination_from_paginator(paginator_page, "Posts");

    let template = TagPostsTemplate {
        page: &page,
        posts: paginator_page.data,
    };

    render_page_with_template(state, &page, template).await
}
