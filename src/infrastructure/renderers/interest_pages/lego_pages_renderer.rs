use askama::Template;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::slug::Slug;
use crate::domain::repositories::LegoRepo;
use crate::domain::services::PageRenderingService;
use crate::domain::{models::page::Page, state::State};

use crate::infrastructure::renderers::RendererContext;
use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "interests/lego_list.html")]
pub struct LegoListTemplate<'t> {
    page: Page,
    total_sets: u32,
    total_pieces: u32,
    sets: Vec<&'t LegoSet>,
    total_minifigs: u32,
    minifigs: Vec<&'t LegoMinifig>,
}

pub async fn render_lego_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/lego"),
        Some("Lego"),
        Some("My Lego Collection".to_string()),
    );

    let template = LegoListTemplate {
        page,
        total_sets: context.data.lego.find_total_sets(),
        total_pieces: context.data.lego.find_total_pieces(),
        sets: context.data.lego.find_all_sets(),
        total_minifigs: context.data.lego.find_total_minifigs(),
        minifigs: context.data.lego.find_all_minifigs(),
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
