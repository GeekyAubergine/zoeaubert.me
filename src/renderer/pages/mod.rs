use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::post::{Post, PostFilter};
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::partials::blog::blog_post_list_item;
use crate::renderer::partials::page::PageComponent;
use crate::renderer::RendererContext;

pub mod home_page_renderer;

pub async fn render_blog_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/blog"), None, None);
    let slug = page.slug.clone();

    // BLOG -

    // Sport Recent - Month - Yeah

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        h1 { ("Blog") }

        ul class="blog-post-list" {
            @for post in &posts {
                    (blog_post_list_item(post))
            }
        }
        // div class="bento home-bento" {
        //     (blog_posts(&context))
        //     (photos(&context))
        //     (exercise_activity(&context))
        //     (exercise_stats_monthly(&context))
        //     (exercise_stats_yearly(&context))
        // }
    };

    let renderer = maud! {
        PageComponent page=(&page) content=(&content);
    };

    context.renderer.render_page(&slug, &renderer, None)
}
