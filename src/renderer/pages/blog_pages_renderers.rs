use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
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
const NOTES_BLOG_POST_TO_IGNORE: &str = "MonthlyNotes";

pub fn render_blog_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => Some(post),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    render_blog_posts_list_page(context, &posts)?;

    for post in posts {
        render_blog_post_page(context, post)?;
    }

    Ok(())
}

pub fn blog_post_list_item<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    let title = maud! {
        h2 class="title" { (&post.title) }
    };

    maud! {
        li class="blog-post-list-item" {
            div class="title-and-date" {
                (link(&post.slug.as_link(), &title))
                (render_date(&post.date))
            }
            p class="description prose" { (post.description )}
            (render_tags(&post.tags, Some(3)))
        }
    }
}

pub fn render_blog_posts_list_page(context: &RendererContext, posts: &Vec<&BlogPost>) -> Result<()> {
    let posts = posts
        .iter()
        .filter(|post| {
            !post.tags
                .iter()
                .any(|t| t.tag().eq(NOTES_BLOG_POST_TO_IGNORE))
        })
        .collect::<Vec<&&BlogPost>>();

    let paginated = paginate(&posts, PAGINATION_SIZE);

    let page = Page::new(Slug::new("/blog"), Some("Blog".to_string()), None);
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let slug = page.slug.clone();

        let content = maud! {
            ul class="blog-post-list" {
                @for post in paginator_page.data {
                    (blog_post_list_item(post))
                }
            }
        };

        let options = PageOptions::new().with_main_class("blog-list-page");

        let renderer = render_page(&page, &options, &content, None);

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}

pub fn render_blog_post_page(context: &RendererContext, post: &BlogPost) -> Result<()> {
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

    let rendered = render_page(&page, &options, &content, None);

    context
        .renderer
        .render_page(&post.slug, &rendered, Some(post.date))
}
