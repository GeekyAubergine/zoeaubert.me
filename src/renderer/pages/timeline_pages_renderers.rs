use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::post::{Post, PostFilter};
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::post_list::render_posts_list;
use crate::renderer::partials::tag::render_tags;
use crate::renderer::partials::utils::link;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_timeline_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::filter_main_timeline())
        .collect::<Vec<&Post>>();

    let paginated = paginate(&posts, PAGINATION_SIZE);

    let page = Page::new(Slug::new("/timeline"), Some("Timeline".to_string()), None);
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let slug = page.slug.clone();

        let content = render_posts_list(paginator_page.data);

        let options = PageOptions::new().with_main_class("timeline-page");

        let renderer = render_page(&page, &options, &content, None);

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
