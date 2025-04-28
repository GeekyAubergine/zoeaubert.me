use askama::Template;

use crate::{
    domain::{
        models::{league::LeagueChampNote, page::Page, slug::Slug},
        repositories::LeagueRepo,
        services::PageRenderingService,
        state::State,
    },
    infrastructure::renderers::RendererContext,
};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_league_pages(context: &RendererContext) -> Result<()> {
    render_champ_notes(context).await
}

#[derive(Template)]
#[template(path = "interests/games/league/league_champ_notes.html")]
pub struct ChampNotesPage<'t> {
    page: Page,
    champ_notes: &'t Vec<LeagueChampNote>,
}

pub async fn render_champ_notes(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games/league/champ-notes"),
        Some("Champ Notes"),
        None,
    );

    let template = ChampNotesPage {
        page,
        champ_notes: &context.data.league.champ_notes,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
