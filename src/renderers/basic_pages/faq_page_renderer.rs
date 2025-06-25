use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::models::{page::Page, slug::Slug},
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
#[template(path = "faq.html")]
struct FaqTemplate<'t> {
    page: Page,
    faq: &'t str,
}

pub async fn render_faq_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("faq"), Some("FAQ"), None);

    let template = FaqTemplate {
        page,
        faq: &context.data.faq.faq,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
