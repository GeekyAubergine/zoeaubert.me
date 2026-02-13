use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub image: Image,
    pub rank: u8,
    pub link: String,
}

#[derive(Clone, Default)]
pub struct Projects {
    projects: HashMap<String, Project>,
}

impl Projects {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn find_all_by_rank_and_name(&self) -> Vec<Project> {
        let mut projects = self.projects.values().cloned().collect::<Vec<Project>>();

        projects.sort_by(|a, b| a.rank.cmp(&b.rank).then_with(|| a.name.cmp(&b.name)));

        projects
    }

    pub fn find_by_name(&self, name: &String) -> Option<Project> {
        self.projects.get(name).cloned()
    }

    pub fn commit(&mut self, project: &Project) {
        self.projects.insert(project.name.clone(), project.clone());
    }
}
