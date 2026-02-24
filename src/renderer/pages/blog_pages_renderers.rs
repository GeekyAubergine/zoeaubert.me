use hypertext::prelude::*;

use crate::domain::models::blog_post::BlogPost;
use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::tag::render_tags;
use crate::renderer::{RenderTask, RenderTasks};
use crate::services::page_renderer::PageRenderer;
use crate::utils::paginator::{Paginator, PaginatorPage};

const PAGINATION_SIZE: usize = 25;
const NOTES_BLOG_POST_TO_IGNORE: &str = "MonthlyNotes";

pub fn render_blog_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    data.timeline_events
        .blog_posts_by_date()
        .for_each(|post| tasks.add(RenderBlogPostPageTask { post }));

    // List pages do additional filtering
    data.timeline_events
        .blog_posts_by_date()
        .filter(|post| {
            !post
                .tags
                .iter()
                .any(|t| t.tag().eq(NOTES_BLOG_POST_TO_IGNORE))
        })
        .paginate(PAGINATION_SIZE)
        .for_each(|paginator_page| {
            tasks.add(RenderBlogPostListPaginatedPageTask { paginator_page })
        });
}

pub fn blog_post_list_item<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    maud! {
        li class="blog-post-list-item" {
            div class="title-and-date" {
                a href=(&post.slug.relative_string()) {
                    h2 class="title" { (&post.title) }
                }
                (render_date(&post.date))
            }
            p class="description prose" { (post.description )}
            (render_tags(&post.tags, Some(3)))
        }
    }
}

struct RenderBlogPostListPaginatedPageTask<'p> {
    paginator_page: PaginatorPage<&'p BlogPost>,
}

impl<'p> RenderTask for RenderBlogPostListPaginatedPageTask<'p> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/blog"), Some("Blog".to_string()), None);

        let page = Page::from_page_and_pagination_page(&page, &self.paginator_page);

        let slug = page.slug.clone();

        let content = maud! {
            ul class="blog-post-list" {
                @for post in &self.paginator_page.data {
                    (blog_post_list_item(post))
                }
            }
        };

        let options = PageOptions::new().with_main_class("blog-list-page");

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}

struct RenderBlogPostPageTask<'p> {
    post: &'p BlogPost,
}

impl<'p> RenderTask for RenderBlogPostPageTask<'p> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let post = self.post;
        let content = maud! {
            article {
                (md(&post.content, md::MarkdownMediaOption::WithMedia))
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

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&post.slug, &rendered, Some(post.date))
    }
}
