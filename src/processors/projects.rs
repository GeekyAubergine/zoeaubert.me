use serde::Deserialize;
use url::Url;

use crate::{
    domain::models::projects::{Project, Projects},
    prelude::*,
    services::{file_service::FilePath, ServiceContext},
};

pub const PROJECTS_FILE: &str = "projects.yml";

#[derive(Debug, Clone, Deserialize, Hash)]
struct FileProject {
    name: String,
    description: String,
    image: Url,
    image_alt: String,
    link: String,
    rank: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectsFile {
    projects: Vec<FileProject>,
}

pub async fn process_projects(ctx: &ServiceContext) -> Result<Projects> {
    let mut projects: Projects = Projects::new();

    let yaml: ProjectsFile = FilePath::content(PROJECTS_FILE).read_as_yaml().await?;

    for file_project in yaml.projects {
        let path = file_project.image.path();

        let image = ctx
            .image
            .copy_image_from_url(
                ctx,
                &file_project.image,
                &FilePath::cache(file_project.image.path()),
                &file_project.image_alt,
            )
            .await?;

        let project = Project {
            name: file_project.name,
            description: file_project.description,
            image,
            rank: file_project.rank,
            link: file_project.link,
        };

        projects.commit(&project);
    }

    Ok(projects)
}
