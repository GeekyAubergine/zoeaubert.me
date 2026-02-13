use hypertext::prelude::*;

use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RendererContext;
use crate::renderer::partials::page::{PageOptions, render_page};

pub fn render_project_pages(context: &RendererContext) -> Result<()> {
    let projects = context.data.projects.find_all_by_rank_and_name();

    let page = Page::new(Slug::new("/projects"), Some("Projects".to_string()), None);

    let slug = page.slug.clone();

    let content = maud! {
        ul {
            @for project in &projects {
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

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}
