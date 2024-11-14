use askama::Template;

use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::slug::Slug;
use crate::domain::services::PageRenderingService;
use crate::domain::{models::page::Page, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "interests/interests_list.html")]
pub struct InterestListTemplate {
    page: Page,
}

pub async fn render_interests_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests"),
        Some("Interests"),
        Some("My Interests"),
    );

    let template = InterestListTemplate { page };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template).await
}
