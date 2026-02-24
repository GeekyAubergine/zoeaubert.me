use hypertext::prelude::*;

use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::projects::Project;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::{RenderTask, RenderTasks};

pub fn render_project_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderProjectsPageTask {
        projects: data.projects.find_all_by_rank_and_name(),
    })
}

struct RenderProjectsPageTask<'p> {
    projects: Vec<&'p Project>,
}

impl<'p> RenderTask for RenderProjectsPageTask<'p> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(Slug::new("/projects"), Some("Projects".to_string()), None);

        let slug = page.slug.clone();

        let content = maud! {
            ul {
                @for project in &self.projects {
                    li {
                        a class="name" href=(project.link) {
                            (project.name)
                        }
                        div class="image" {
                            (project.image.render_large())
                        }
                        p { (project.description) }
                    }
                }
            }
        };

        let options = PageOptions::new().with_main_class("projects-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
