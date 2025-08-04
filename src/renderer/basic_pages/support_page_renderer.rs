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
#[template(path = "support.html")]
struct SaveTemplate {
    page: Page,
}

pub async fn render_support_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("support"), Some("Support"), None);

    let template = SaveTemplate { page };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
