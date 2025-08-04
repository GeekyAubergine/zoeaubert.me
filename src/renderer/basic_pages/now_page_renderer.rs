use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::models::{page::Page, post::Post, referral::Referral, slug::Slug},
    prelude::*,
    renderers::RendererContext,
    utils::paginator::{paginate, PaginatorPage},
};

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

#[derive(Template)]
#[template(path = "now.html")]
struct FaqTemplate<'t> {
    page: Page,
    now: &'t str,
}

pub async fn render_now_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("now"), Some("Now"), None);

    let template = FaqTemplate {
        page,
        now: &context.data.now_text.now_text,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
