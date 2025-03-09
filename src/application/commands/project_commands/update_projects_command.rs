use std::path::Path;

use serde::Deserialize;
use url::Url;

use crate::domain::models::image::Image;
use crate::domain::models::project::Project;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::ProjectsRepo;
use crate::domain::services::{FileService, ImageService};
use crate::domain::state::State;
use crate::{calculate_hash, prelude::*};

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

pub async fn update_projects_command(state: &impl State) -> Result<()> {
    let yaml: ProjectsFile = state
        .file_service()
        .read_yaml_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(PROJECTS_FILE)),
        )
        .await?;

    for file_project in yaml.projects {
        let hash = calculate_hash(&file_project);

        if let Some(existing) = state
            .projects_repo()
            .find_by_name(&file_project.name)
            .await?
        {
            if hash == existing.original_data_hash {
                continue;
            }
        }

        let path = file_project.image.path();

        let image = state
            .image_service()
            .copy_image_from_url(
                state,
                &file_project.image,
                Path::new(file_project.image.path()),
                &file_project.image_alt,
            )
            .await?;

        let project = Project {
            name: file_project.name,
            description: file_project.description,
            image,
            rank: file_project.rank,
            link: file_project.link,
            original_data_hash: hash,
        };

        state.projects_repo().commit(&project).await?;
    }

    Ok(())
}
