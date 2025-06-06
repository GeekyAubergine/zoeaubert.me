use askama::Template;

use crate::domain::models::page::Page;
use crate::domain::models::projects::Project;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::{BlogPostsRepo, ProjectsRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::infrastructure::renderers::RendererContext;
use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

pub async fn render_project_pages(context: &RendererContext) -> Result<()> {
    let projects = context.data.projects.find_all_by_rank_and_name();

    render_projects_list_page(context, &projects).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "projects/projects_list.html")]
struct ProjectListTemplate {
    page: Page,
    projects: Vec<Project>,
}

async fn render_projects_list_page(context: &RendererContext, projects: &[Project]) -> Result<()> {
    let page = Page::new(
        Slug::new("/projects"),
        Some("Projects"),
        Some("My projects".to_string()),
    );

    let template = ProjectListTemplate {
        page,
        projects: projects.to_vec(),
    };

    context.renderer.render_page(&template.page.slug, &template, None).await
}
