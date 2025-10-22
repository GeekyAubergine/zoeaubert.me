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
use crate::renderer::partials::tag::render_tags;
use crate::renderer::partials::utils::link;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 25;

pub fn render_posts_pages(context: &RendererContext) -> Result<()> {
    let posts = context.data.posts.find_all_by_date_iter();

    for post in posts {
        render_post_page(context, post)?;
    }

    Ok(())
}

pub fn render_post_page(context: &RendererContext, post: &Post) -> Result<()> {
    match post {
        Post::BlogPost(post) => render_blog_post_page(context, post),
        _ => Ok(()),
    }
}

pub fn render_blog_post_page(context: &RendererContext, post: &BlogPost) -> Result<()> {
    let content = maud! {
        article {
            (md(&post.content))
        }
    };

    let options = PageOptions::new().with_main_class("blog-post-page");

    let page = Page::new(
        post.slug.clone(),
        Some(post.title.clone()),
        Some(post.description.clone()),
    )
    .with_date(post.date)
    .with_tags(post.tags.clone());
    // .with_image(post.hero_image); TODO

    let rendered = render_page(&page, &options, &content, None);

    context
        .renderer
        .render_page(&post.slug, &rendered, Some(post.date))
}
