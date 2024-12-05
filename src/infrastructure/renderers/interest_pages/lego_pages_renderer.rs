use askama::Template;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::slug::Slug;
use crate::domain::repositories::LegoRepo;
use crate::domain::services::PageRenderingService;
use crate::domain::{models::page::Page, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "interests/lego_list.html")]
pub struct LegoListTemplate {
    page: Page,
    total_sets: u32,
    total_pieces: u32,
    sets: Vec<LegoSet>,
    total_minifigs: u32,
    minifigs: Vec<LegoMinifig>,
}

pub async fn render_lego_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/lego"),
        Some("Lego"),
        Some("My Lego Collection"),
    );

    let template = LegoListTemplate {
        page,
        total_sets: state.lego_repo().find_total_sets().await?,
        total_pieces: state.lego_repo().find_total_pieces().await?,
        sets: state.lego_repo().find_all_sets().await?,
        total_minifigs: state.lego_repo().find_total_minifigs().await?,
        minifigs:state.lego_repo().find_all_minifigs().await?,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None).await
}
