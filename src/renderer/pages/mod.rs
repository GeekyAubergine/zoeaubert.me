use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::page::Page;
use crate::domain::models::post::{Post, PostFilter};
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::partials::blog::blog_post_list_item;
use crate::renderer::partials::page::{PageComponent, PageOptions, PageWidth};
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

pub mod home_page_renderer;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_blog_page(context: &RendererContext) -> Result<()> {
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

    let paginated = paginate(&posts, DEFAULT_PAGINATION_SIZE);

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
            // div class="bento home-bento" {
            //     (blog_posts(&context))
            //     (photos(&context))
            //     (exercise_activity(&context))
            //     (exercise_stats_monthly(&context))
            //     (exercise_stats_yearly(&context))
            // }
        };

        let options = PageOptions::new().with_width(PageWidth::Narrow);

        let renderer = maud! {
            PageComponent page=(&page) options=(&options) content=(&content);
        };

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
