use askama::Template;

use crate::domain::models::page::Page;
use crate::domain::models::project::Project;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::{BlogPostsRepo, ProjectsRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_project_pages(state: &impl State) -> Result<()> {
    let projects = state.projects_repo().find_all_by_rank_and_name().await?;

    render_projects_list_page(state, &projects).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "projects/projects_list.html")]
struct ProjectListTemplate {
    page: Page,
    projects: Vec<Project>,
}

async fn render_projects_list_page(state: &impl State, projects: &[Project]) -> Result<()> {
    let page = Page::new(
        Slug::new("/projects"),
        Some("Projects"),
        Some("My projects".to_string()),
    );

    let template = ProjectListTemplate {
        page,
        projects: projects.to_vec(),
    };

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            None,
        )
        .await
}
