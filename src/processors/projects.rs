use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    domain::models::projects::{Project, Projects},
    prelude::*,
    services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{FileService, ReadableFile},
        media_service::MediaService,
    },
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

pub fn load_projects(ctx: &ServiceContext) -> Result<Projects> {
    info!("Processing Projects");
    let mut projects: Projects = Projects::new();

    let yaml: ProjectsFile = FileService::content(PROJECTS_FILE.into()).read_yaml()?;

    for file_project in yaml.projects {
        let path = file_project.image.path();

        let cdn_file = CdnFile::from_str(path);

        let image = MediaService::image_from_url(
            ctx,
            &file_project.image,
            &cdn_file,
            &file_project.image_alt,
            None,
            None,
        )?;

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
