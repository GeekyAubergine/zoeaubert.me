use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::{
        models::{
            movie::{MovieId, MovieReview},
            project::Project,
            slug::Slug,
        },
        repositories::{MovieReviewsRepo, ProjectsRepo}, services::FileService,
    },
    infrastructure::services::file_service_disk::FileServiceDisk,
    prelude::*,
};

const FILE_NAME: &str = "projects.json";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ProjectsRepoData {
    projects: HashMap<Slug, Project>,
}

pub struct ProjectsRepoDisk {
    data: Arc<RwLock<ProjectsRepoData>>,
    file_service: FileServiceDisk,
}

impl ProjectsRepoDisk {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl ProjectsRepo for ProjectsRepoDisk {
    async fn find_all_by_rank_and_name(&self) -> Result<Vec<Project>> {
        let mut projects = self
            .data
            .read()
            .await
            .projects
            .values()
            .cloned()
            .collect::<Vec<Project>>();

        projects.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.name.cmp(&b.name)));

        Ok(projects)
    }

    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Project>> {
        let data = self.data.read().await;
        Ok(data.projects.get(slug).cloned())
    }

    async fn commit(&self, project: &Project) -> Result<()> {
        let mut data = self.data.write().await;
        data.projects.insert(project.slug.clone(), project.clone());

        self.file_service
            .write_json_file(&make_file_path(&self.file_service), &data.clone())
            .await?;

        Ok(())
    }
}
