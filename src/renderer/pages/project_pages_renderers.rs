use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

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
