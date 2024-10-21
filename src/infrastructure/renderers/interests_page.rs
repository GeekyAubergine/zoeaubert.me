use askama::Template;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::slug::Slug;
use crate::domain::{models::page::Page, state::State};

use crate::prelude::*;

use super::render_page_with_template;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "interests/interests_list.html")]
pub struct InterestListTemplate<'t> {
    page: &'t Page<'t>,
}

pub async fn render_interests_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests"),
        Some("Interests"),
        Some("My Interests"),
    );

    let template = InterestListTemplate { page: &page };

    render_page_with_template(state, &page, template).await
}
