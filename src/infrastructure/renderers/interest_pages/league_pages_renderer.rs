use askama::Template;

use crate::domain::{
    models::{league::LeagueChampNote, page::Page, slug::Slug},
    repositories::LeagueRepo,
    services::PageRenderingService,
    state::State,
};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_league_pages(state: &impl State) -> Result<()> {
    render_champ_notes(state).await
}

#[derive(Template)]
#[template(path = "interests/games/league/league_champ_notes.html")]
pub struct ChampNotesPage {
    page: Page,
    champ_notes: Vec<LeagueChampNote>,
}

pub async fn render_champ_notes(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests/games/league/champ-notes"),
        Some("Champ Notes"),
        None,
    );

    state
        .page_rendering_service()
        .add_page(
            state,
            page.slug.clone(),
            ChampNotesPage {
                page,
                champ_notes: state.league_repo().find_all_champ_notes_by_name().await?,
            },
            None,
        )
        .await
}
