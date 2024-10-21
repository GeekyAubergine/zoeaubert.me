use askama::Template;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::slug::Slug;
use crate::domain::queries::lego_queries::{find_all_lego_sets, find_all_lego_minifiigs, find_total_lego_minifiigs, find_total_lego_pieces, find_total_lego_sets};
use crate::domain::{models::page::Page, state::State};

use crate::prelude::*;

use super::render_page_with_template;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "interests/lego_list.html")]
pub struct LegoListTemplate<'t> {
    page: &'t Page<'t>,
    total_sets: u32,
    total_pieces: u32,
    sets: &'t [LegoSet],
    total_minifigs: u32,
    minifigs: &'t [LegoMinifig],
}

pub async fn render_lego_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/lego"),
        Some("Lego"),
        Some("My Lego Collection"),
    );

    let template = LegoListTemplate {
        page: &page,
        total_sets: find_total_lego_sets(state).await?,
        total_pieces: find_total_lego_pieces(state).await?,
        sets: &find_all_lego_sets(state).await?,
        total_minifigs: find_total_lego_minifiigs(state).await?,
        minifigs: &find_all_lego_minifiigs(state).await?,
    };

    render_page_with_template(state, &page, template).await
}
